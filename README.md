# mince
Mince comes from "mincing" which means taking something that is roughly chopped and then chopping it finely

> **Warning** This package is under heavy development

## Usage

1. Install package

```bash
npm install @whizzes/mince
```

2. Initialize WASM Module

```ts
import init from "@whizzes/mince";
```

3. Then use the `Mince` class

```ts
import { Mince } from "@whizzes/mince";

const resizeImage = (file: File): Promise<File> => {
    const mince: Mince = await Mince.fromFile(file);
    const resized: Mince = mince.resize(100, 100);
    const file: File = resized.toFile();

    // The resulting file is an instance of the Browser's native `File` object
    const url = URL.createObjectURL(file);

    document.getElementById('image').src = url;

    return file;
}
```

## Development

1. Install `wasm-pack`

```bash
cargo install wasm-pack
```

## Geckodriver

https://github.com/rustwasm/wasm-pack/blob/master/src/test/webdriver/geckodriver.rs
