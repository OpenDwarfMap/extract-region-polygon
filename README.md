A library to extract the outside border of Dwarf Fortress regions from the string defining the squares contained in the region given by DFHack.
Made to be used as a WebAssembly module, built with wasm-pack.

## Build

### Building for front-end

To build the project to be used in your front-end application using a bundler, use:

```
wasm-pack build
```

### Building for node

To build the project to be used in a back-end application using Node.JS, use:

```
wasm-pack build --target nodejs
```
