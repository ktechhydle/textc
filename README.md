# `textc`
**Compress and decompress UTF-8 text with speed and ease.**

> [!WARNING]
> `textc` is not a production-grade compression tool. It is purely a personal project (for now).

## How It Works
`textc` compresses repeating text sequences using a dictionary + contents approach. The following steps are done in sequence to produce a binary `.tzp` file.

1. Split text into spaces, newlines, and tabs -> `"hello hello world"` becomes `["hello", " ", "hello", " ", "world"]`
2. Remove any duplicate sequences -> `["hello", " ", "hello", " ", "world"]` becomes `["hello", " ", "world"]`
3. Assign an ID to each index of the sequence forming a table -> `["hello", " ", "world"]` has corresponding IDs `[0, 1, 2]`
4. Append the ID set + split text set into a binary format

To decompress, `textc` will look at the corresponding ID in the ID set, then map that to the split text set, therefore reconstructing the data.

## Pros And Cons
`textc` is optimized for repeating patterns in text, meaning compressing a source code file might result in a higher final size compared to compressing something like an essay.

**Pros**:
- Extremely simple algorithm
- Fast compression and decompression due to direct ID lookup
- Great for highly repetitive natural language text (essays, transcripts, logs)
- Performs well on structured text like HTML/XML with repeated tags
- Deterministic output (same input → same binary structure)
- Lightweight implementation compared to complex algorithms like gzip or 7-Zip
- Potentially useful for preprocessing large text corpora before ML training

**Cons**:
- Poor compression on low-repetition data (e.g., many source code files)
- No entropy encoding (like Huffman or arithmetic coding), so storage is not bit-optimal
- No sliding window mechanism like LZ77
- Dictionary overhead can outweigh gains on small files
- Not competitive with mature compressors like Zstandard
- No built-in streaming support (whole file needs to be processed at once)
- UTF-8 token splitting by whitespace may miss deeper repetition patterns
- Not suitable for binary files
