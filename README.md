# mdbook-qa

## Usage

**Configuration in book.toml**
```toml
[preprocessor.qa]
```

**Markdown usage**
```markdown
Some text here.

Q: What is the answer to the Ultimate Question of Life, the Universe, and Everything?
A: 42.
QAEND

or <QAEND>

And next Q&A:

Q: What does “const X* p” mean?

A: It means p points to an object of class X, but p can’t 
be used to change that X object (naturally p could also be NULL).

*another line*

Read it right-to-left: “p is a pointer to an X that is constant.”

There is no need to write QAEND at the end of the file.
```

![Image](https://github.com/PyXiion/mdbook-qa/assets/59997570/e7acf4a0-acdc-4914-b1c5-0f26238546c9)
