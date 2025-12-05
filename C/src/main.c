#define _POSIX_C_SOURCE 200809L
#include "lz77.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static void print_usage(const char *prog)
{
    fprintf(stderr,
        "Usage: %s [OPTIONS]\n"
        "\n"
        "Options:\n"
        "  -c          Compress (default)\n"
        "  -d          Decompress\n"
        "  -i FILE     Read input from FILE\n"
        "  -s STRING   Use STRING as input\n"
        "  -o FILE     Write output to FILE (default: stdout)\n"
        "  -h          Show this help\n"
        "\n"
        "Examples:\n"
        "  %s -s \"hello world\" -o out.lz77\n"
        "  %s -i input.txt -o compressed.lz77\n"
        "  %s -d -i compressed.lz77 -o output.txt\n",
        prog, prog, prog, prog);
}

static uint8_t * read_file(const char * filename, size_t * out_len)
{
    FILE * fp = fopen(filename, "rb");
    if (!fp)
    {
        perror(filename);
        return NULL;
    }

    fseek(fp, 0, SEEK_END);
    long filesz = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    if (filesz < 0)
    {
        perror("ftell");
        fclose(fp);
        return NULL;
    }

    uint8_t * data = malloc(((size_t) filesz) * sizeof(uint8_t));
    if (!data)
    {
        fprintf(stderr, "Out of memory\n");
        fclose(fp);
        return NULL;
    }

    if (fread(data, 1, (size_t) filesz, fp) != ((size_t) filesz))
    {
        perror("fread");
        free(data);
        fclose(fp);
        return NULL;
    }

    fclose(fp);
    *out_len = (size_t) filesz;
    return data;
}

static int32_t write_file(const char * filename, const uint8_t * data, size_t len)
{
    FILE * fp = fopen(filename, "wb");
    if (!fp)
    {
        perror(filename);
        return -1;
    }

    if (fwrite(data, 1, len, fp) != len)
    {
        perror("fwrite");
        fclose(fp);
        return -1;
    }

    fclose(fp);
    return 0;
}

static int32_t write_stdout(const uint8_t * data, size_t len)
{
    if (fwrite(data, 1, len, stdout) != len)
    {
        perror("fwrite");
        return -1;
    }
    return 0;
}

int main(int argc, char ** argv)
{
    int32_t decompress = 0;

    const char * input_filename  = NULL;
    const char * input_string    = NULL;
    const char * output_filename = NULL;

    int opt;
    while ((opt = getopt(argc, argv, "cdi:s:o:h")) != -1)
    {
        switch (opt)
        {
            case 'c':
                decompress = 0;
                break;
            case 'd':
                decompress = 1;
                break;
            case 'i':
                input_file = optarg;
                break;
            case 's':
                input_string = optarg;
                break;
            case 'o':
                output_file = optarg;
                break;
            case 'h':
                print_usage(argv[0]);
                return 0;
            default:
                print_usage(argv[0]);
                return 1;
        }
    }

    /* Validate input options */
    if (input_filename && input_string)
    {
        fprintf(stderr, "Error: cannot use both -i and -s\n");
        return 1;
    }
    if (!input_filename && !input_string)
    {
        fprintf(stderr, "Error: must specify -i or -s\n");
        print_usage(argv[0]);
        return 1;
    }

    /* Get input data */
    uint8_t * input = NULL;
    size_t input_len = 0;
    int free_input = 0;

    if (input_file)
    {
        input = read_file(input_filename, &input_len);
        if (!input)
        {
            return 1;
        }
        free_input = 1;
    }
    else
    {
        input = (uint8_t *)input_string;
        input_len = strlen(input_string);
    }

    lz77_buffer_t output = { 0 };

    int rc;

    if (decompress)
    {
        rc = lz77_decompress(input, input_len, &output);
    }
    else
    {
        lz77_config_t cfg = { 0 };
        lz77_config_init(&cfg);
        result = lz77_compress(input, input_len, &output, &cfg);
    }

    if (free_input)
    {
        free(input);
    }

    if (rc != LZ77_OK)
    {
        const char * msg = NULL;
        switch (rc)
        {
            case LZ77_ERR_NOMEM:
                msg = "Out of memory";
                break;
            case LZ77_ERR_INVALID:
                msg = "Invalid compressed data";
                break;
            default:
                msg = "Unknown error";
        }
        fprintf(stderr, "Error: %s\n", msg);
        return 1;
    }

    if (output_file)
    {
        rc = write_file(output_filename, output.data, output.len);
    }
    else
    {
        rc = write_stdout(output.data, output.len);
    }
    lz77_buffer_free(&output);
    return (rc == 0) ? 0 : 1;
}
