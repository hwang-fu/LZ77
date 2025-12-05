#pragma once

#include <stddef.h>
#include <stdint.h>

/* Default compression parameters */
#define LZ77_WINDOW_SIZE     (4096)
#define LZ77_MAX_MATCH_LEN   (258)
#define LZ77_MIN_MATCH_LEN   (3)

/* Token types */
#define LZ77_TOKEN_LITERAL   (0x00)
#define LZ77_TOKEN_REFERENCE (0x01)

/* Result codes */
#define LZ77_OK              (0)
#define LZ77_ERR_NOMEM       (-1)
#define LZ77_ERR_INVALID     (-2)
#define LZ77_ERR_IO          (-3)

/* Compression context */
typedef struct {
    size_t window_size;
    size_t min_match;
    size_t max_match;
} lz77_config_t;

/* Compressed data buffer */
typedef struct {
    uint8_t * data;
    size_t    len;
    size_t    capacity;
} lz77_buffer_t;

/* Initialize config with defaults */
void lz77_config_init(lz77_config_t * cfg);

/* Compress input data, caller must free output->data */
int32_t lz77_compress(
    const uint8_t * input,
    size_t input_len,
    lz77_buffer_t * output,
    const lz77_config_t * cfg
);

/* Decompress data, caller must free output->data */
int32_t lz77_decompress(const uint8_t * input, size_t input_len, lz77_buffer_t * output);

/* Free buffer data */
void lz77_buffer_free(lz77_buffer_t * buf);
