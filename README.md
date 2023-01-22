# Yomi-Reader

Simple web app to read Japanese ebooks in EPUB format with one-click dictionary lookup.
It is compatible with [Yomichan Dictionaries](https://github.com/FooSoft/yomichan/#dictionaries).

This is not meant to replace Yomichan -- it is in fact largely worse!
However, where using Yomichan on mobile devices is not convenient, this may be more accessible.

## Features

- Import EPUB books and Yomichan dictionaries
- Remember chapter and reading position in page
- Look up terms with one tap while taking inflections into account
- Completely local

**Note** that you currently cannot reorder or delete imported dictionaries, and disrupting an import will prevent a full import of the given dictionary. 
In that case, a simple clearing of the IndexedDB database is advised before reimporting the dictionaries in the desired order.
