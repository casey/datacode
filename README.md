datacode
========

Datacode is a proposal for a visually compact encoding of binary data in plain
text.

It proposes allocating the 65,536 Unicode plane four code points,
U+40000–4FFFF, to visual representations of all possible 16-bit values, and
code points U+1FF00–1FFFF to visual representations of all possible 8-bit
values.

These two character ranges would allow visually compact plain-text
representations of binary data. With code points U+40000–4FFFF the leading
pairs of bytes, with an optional U+1FF00–1FFFF code point for the last byte, if
the number of bytes is odd.

motivation
----------

Text representations of binary data are ubiquitous, with a great variety of
different representations commonly in use. Uses include cryptographic hashes,
public keys, web content IDs, for example, YouTube video IDs, and many more.

Examples of binary-to-text encoding schemes include:

- Hexadecimal, which uses characters in the set `[0-9a-f]` to encode four bits.

- Base64, which uses a variety of 64 character sets to encode six bits.
  `[0-9a-zA-Z+/]`, is a common choice of characters, but many variations exist.
  Since each characters encodes six bits, a padding character, commonly =, is
  sometimes used to indicate that the final bits should be discarded after
  encoding.

- bech32, primarily used to encode Bitcoin addresses, which implements a BCH
  code over the characters [qpzry9x8gf2tvdw0s3jn54khce6mua7l], with each
  character encoding five bits.

However, no character set dedicated to representing binary data exists.

proposal
--------

Code points U+40000–4FFFF are allocated to visual representations of all 65,536
possible two-byte values, and are called "paircodes". Code points 1FF00–1FFFF
are allocated to visual representations of all 256 one byte values, and are
called "bytecodes".

An N byte sequence can then be represented with N / 2 paircode characters,
followed by a single bytecode if N is odd.

Binary data encoded as datacode uses 75% fewer characters than hexadecimal, and
62.5% fewer characters than Base64.

The same 32 byte hash encoded as hex:

```
4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
```

Versus Base64:

```
Sl4eS6q4nzoyUYqIwxvIf2GPdmc+LMd6shJ7ev3tozs=
```

Versus datacode, using the `❑` character as a placeholder:

```
❑❑❑❑❑❑❑❑❑❑❑❑❑❑❑❑
```

rendering
---------

Each paircode is represented as a four-by-four grid of cells, with an empty
cell representing the binary digit 0, and a filled cell representing the binary
digit 1:

```
┏━━━━┓
┃....┃
┃....┃
┃....┃
┃....┃
┗━━━━┛
```

Each column of a paircode grid represents 4 bits, with columns arranged left to
right from least significant to most significant, and bits within columns
arranged top to bottom from least significant to most significant.

Each bytecode is represented similarly as a two-by-four grid of cells:

```
┏━━┓
┃..┃
┃..┃
┃..┃
┃..┃
┗━━┛
```

The three bytes encoded in hex as `4a5e1e`:

```
┏━━━━┓┏━━┓
┃..█.┃┃█.┃
┃.█.█┃┃.█┃
┃█.██┃┃.█┃
┃.█.█┃┃.█┃
┗━━━━┛┗━━┛
```

Filled cells are represented as solid squares, and empty cells as empty
squares. Character borders can be included or omitted.

Fonts for datacode characters are easy to produce, since all characters can be
generated programmatically. Presumably, not all fonts would include datacode
characters, and rendering would be handled by a specialized fallback font for
the code point ranges.

applications
------------

Datacode can be used to to compactly represent binary data in applications
which expose binary data in text to the end user. It is not appropriate for use
in non user-facing applications, since the actual encoding of datacode as
UTF-8, is larger than the equivalent binary or hex encoding, given the size of
the datacode code points.

remarks
-------

A substantial drawback of datacode is that it cannot be easily be typed by a
less than highly motivated user. However, in nearly all applications using text
representations of binary data, in nearly all cases, those representations are
not intended to be typed, and are much more likely to simply be present in URLs
and other places where the primary form of interaction is to copy and paste the
text.

Datacode's more compact encoding of binary data allows it to be more easily
copied and pasted, and fit more compactly within other text.

Additionally, text interfaces could provide for special handling of datacode
sequences, for example, making a single click anywhere within the sequence
select the entire contiguous sequence of datacode characters.

Datacode could also improve accessibility, by clearly delineating
human-readable text from text representations of binary data.
