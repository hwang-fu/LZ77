#include "lz77.h"
#include <stdlib.h>
#include <string.h>

/* Initial buffer capacity */
#define LZ77_BUFFER_INITIAL_CAPACITY 1024

/* Ensure buffer has room for n more bytes */
static int32_t lz77_buffer_grow(lz77_buffer_t * buf, size_t n)
{
    if (!buf)
    {
        return LZ77_ERR_INVALID;
    }

    if (buf->len + n <= buf->capacity)
    {
        return LZ77_OK;
    }

    size_t new_capacity = buf->capacity * 2 + 1;
    while (new_capacity <= buf->len + n)
    {
        new_capacity *= 2;
    }

    uint8_t * new_data = realloc(buf->data, new_capacity * sizeof(uint8_t));
    if (!new_data)
    {
        return LZ77_ERR_NOMEM;
    }

    buf->data     = new_data;
    buf->capacity = new_capacity;
    return LZ77_OK;
}

/* Push a single byte to buffer */
static int32_t lz77_buffer_push(lz77_buffer_t * buf, uint8_t byte)
{
    if (!buf)
    {
        return LZ77_ERR_INVALID;
    }

    if (lz77_buffer_grow(buf, 1) != LZ77_OK)
    {
        return LZ77_ERR_NOMEM;
    }

    buf->data[buf->len++] = byte;

    return LZ77_OK;
}

/* Append multiple bytes to buffer */
static int32_t lz77_buffer_append(lz77_buffer_t * buf, uint8_t * data, size_t n)
{
    if (!buf)
    {
        return LZ77_ERR_INVALID;
    }

    if (lz77_buffer_grow(buf, n) != LZ77_OK)
    {
        return LZ77_ERR_NOMEM;
    }

    memcpy(buf->data + buf->len, data, n);
    buf->len += n;

    return LZ77_OK;
}

static void lz77_find_match(
    const uint8_t * data, size_t data_len,
    size_t pos, size_t window_size, size_t min_match, size_t max_match,
    size_t * out_offset, size_t * out_length
)
{
    size_t search_start = (pos > window_size) ? pos - window_size : 0;
    size_t rest         = data_len - pos;
    size_t max_len      = (rest < max_match) ? rest : max_match;

    if (max_len < min_match)
    {
        return;
    }

    for (size_t i = search_start; i < pos; i++)
    {
        size_t match_len = 0;

        while (
            match_len < max_len &&
            data[i + match_len] == data[pos + match_len]
        )
        {
            match_len++;
        }

        if (match_len >= min_match && match_len > *out_length)
        {
            *out_offset = pos - i;
            *out_length = match_len;

            if (match_len == max_len)
            {
                break; // can't do better
            }
        }
    }
}

void lz77_config_init(lz77_config_t * cfg)
{
    if (cfg)
    {
        cfg->window_size = LZ77_WINDOW_SIZE;
        cfg->min_match   = LZ77_MIN_MATCH_LEN;
        cfg->max_match   = LZ77_MAX_MATCH_LEN;
    }
}

int32_t lz77_compress(
    const uint8_t * input,
    size_t input_len,
    lz77_buffer_t * output,
    const lz77_config_t * cfg
)
{
    output->data = malloc(LZ77_BUFFER_INITIAL_CAPACITY * sizeof(uint8_t));
    if (!output->data)
    {
        return LZ77_ERR_NOMEM;
    }
    output->len = 0;
    output->capacity = LZ77_BUFFER_INITIAL_CAPACITY;

    size_t pos = 0;

    while (pos < input_len)
    {
        size_t offset = 0;
        size_t length = 0;
        lz77_find_match(
            input, input_len,
            pos, cfg->window_size, cfg->min_match, cfg->max_match,
            &offset, &length
        );

        if (length >= cfg->min_match)
        {
            /* Emit reference token: [0x01][offset:2][length:2] */
            uint8_t token[5];
            token[0] = LZ77_TOKEN_REFERENCE;
            token[1] = (offset >> 8) & 0xFF;
            token[2] = offset & 0xFF;
            token[3] = (length >> 8) & 0xFF;
            token[4] = length & 0xFF;

            if (lz77_buffer_append(output, token, 5) != LZ77_OK)
            {
                goto fail;
            }

            pos += length;
        }
        else
        {
            /* Emit literal token: [0x00][byte] */
            if (lz77_buffer_push(output, LZ77_TOKEN_LITERAL) != LZ77_OK)
            {
                goto fail;
            }
            if (lz77_buffer_push(output, input[pos]) != LZ77_OK)
            {
                goto fail;
            }
            pos++;
        }
    }

    return LZ77_OK;

fail:
    lz77_buffer_free(output);
    return LZ77_ERR_NOMEM;
}

int32_t lz77_decompress(const uint8_t * input, size_t input_len, lz77_buffer_t * output)
{
    output->data = malloc(LZ77_BUFFER_INITIAL_CAPACITY * sizeof(uint8_t));
    if (!output->data)
    {
        return LZ77_ERR_NOMEM;
    }
    output->len = 0;
    output->capacity = LZ77_BUFFER_INITIAL_CAPACITY;

    size_t pos = 0;

    while (pos < input_len)
    {
        const uint8_t MAGIC_NUMBER = input[pos++];

        if (MAGIC_NUMBER == LZ77_TOKEN_LITERAL)
        {
            if (pos >= input_len)
            {
                goto invalid;
            }
            if (lz77_buffer_push(output, input[pos++]) != LZ77_OK)
            {
                goto fail;
            }
        }
        else if (MAGIC_NUMBER == LZ77_TOKEN_REFERENCE)
        {
            if (pos + 4 > input_len)
            {
                goto invalid;
            }

            size_t offset = ((size_t)input[pos] << 8)     | input[pos + 1];
            size_t length = ((size_t)input[pos + 2] << 8) | input[pos + 3];
            pos += 4;

            if (offset > output->len || offset == 0)
            {
                goto invalid;
            }

            size_t start = output->len - offset;
            for (size_t i = 0; i < length; i++)
            {
                if (lz77_buffer_push(output, output->data[start + i]) != LZ77_OK)
                {
                    goto fail;
                }
            }
        }
        else
        {
            goto invalid;
        }
    }

    return LZ77_OK;

invalid:
    lz77_buffer_free(output);
    return LZ77_ERR_INVALID;

fail:
    lz77_buffer_free(output);
    return LZ77_ERR_NOMEM;
}

void lz77_buffer_free(lz77_buffer_t * buf)
{
    if (!buf)
    {
        return;
    }

    if (buf->data)
    {
        free(buf->data);
        buf->data = NULL;
    }
    buf->len      = 0;
    buf->capacity = 0;
}
