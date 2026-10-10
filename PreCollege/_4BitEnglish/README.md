
# Note:

This is a failed crate that is half baked.
The Reason I included this project here is because 
I learned a lesson here about computer architecture, and the time-memory trade off.
That being Sub-byte packing, while sounds great on paper, it required accessing data extremely sequential and decoding logic in a vectorized or microcoded form, I would lose far more in CPU time and cache efficiency than I would save in memory. Even more so for text active text editing would lose me more memory and CPU time then using the less cool classic byte per char.

# 4bitEnglish
Rust crate for a compact 16-character subset of English that fits perfectly into 4 bits per character (2 chars per byte). 
I made this to drastically reduce memory usage for strings in constrained environments: kernels, embedded, toy OSes, logs, config files...

Everything else is substituted with digraphs or closest sounds.

## Allowed Characters

| Category     | Characters                          |
|--------------|-------------------------------------|
| Vowels       | `a e i o u`                         |
| Consonants   | `t n s h r d l m p k`               |
| Whitespace   | ` ` (space)                         |

## Substitution Rules

| Original | → 4bitEnglish   |                      |
|----------|------------------|---------------------------|
| b        | p                | closest voiceless         |
| c        | k / s            | hard c → k, soft → s      |
| f        | ph               |                           |
| g        | k / d            | hard → k, soft → d        |
| j        | d                |                           |
| q        | k                |                           |
| v        | p                |                           |
| w        | u                |                           |
| x        | ks               |                           |
| y        | i                |                           |
| z        | s / ed           | z → s, or ed in endings   |

## Quick Example

**Original**  
Error: failed to allocate buffer for process.

**4bitEnglish**  
Error: phailed to alokate puffer phor proses.
