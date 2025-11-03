# Changelog

All notable changes to this project will be documented in this file.

## [0.13.0](https://github.com/GrayJack/bitflag-attr/compare/v0.12.1..v0.13.0) — 2025-11-03

### 🚀 Features

- Re-export `serde_core` items and use on them on the generated code — ([3e3b2bc](https://github.com/GrayJack/bitflag-attr/commit/3e3b2bc5dd534ed49c15e8c1a7ad42d81f3d0539))

### 🚜 Refactor

- Refactor attribute macro to not depend on the `full` feature of `syn` — ([1d47355](https://github.com/GrayJack/bitflag-attr/commit/1d47355c9841cb4ee69c73f55fb601b451a6d9b3))

## [0.12.1](https://github.com/GrayJack/bitflag-attr/compare/v0.12.0..v0.12.1) — 2025-03-13

### 🐞 Bug Fixes

- Fix code produced with `const-mut-ref` feature — ([4148b4a](https://github.com/GrayJack/bitflag-attr/commit/4148b4a80d9e3a4f17933618f14ef78eabec2a1f))

## [0.12.0](https://github.com/GrayJack/bitflag-attr/compare/v0.11.1..v0.12.0) — 2025-02-21

### 🚀 Features

- Add `all_named` and `is_all_named` to the `Flags` trait — ([b472ff5](https://github.com/GrayJack/bitflag-attr/commit/b472ff5403c221467655951a7a15f5dbae33d17b))
- Add custom error for when the enum have fields — ([4be6862](https://github.com/GrayJack/bitflag-attr/commit/4be6862ca6b02cb0f96281a3bc16f46e144c4fab))

### 🚜 Refactor

- (**BREAKING**) Rename `Flags::KNOWN_FLAGS` to `Flags::NAMED_FLAGS` — ([8d298d9](https://github.com/GrayJack/bitflag-attr/commit/8d298d93fab1567bcc1b92737326c180cfea4e01))
- Reduce a bit the size of the macro typed information — ([c5b79d5](https://github.com/GrayJack/bitflag-attr/commit/c5b79d5f5e1d6338fd55fdb67d9cdf1b9c949c94))
- (**BREAKING**) Rename `extra_valid_bits` to `reserved_bits` — ([9a703a2](https://github.com/GrayJack/bitflag-attr/commit/9a703a2e2bc503c5b7da43cb22ebf8c5d576ce20))

### 📚 Documentation

- Add the changelog to the crate documentation — ([4f59a54](https://github.com/GrayJack/bitflag-attr/commit/4f59a54056f9d2a593f0d556844f28b8e53e5627))
- A few rewording — ([e35bf5f](https://github.com/GrayJack/bitflag-attr/commit/e35bf5f13085a3e584e514fa9baba5a5690adf1f))
- Add specification documentation — ([67c5cde](https://github.com/GrayJack/bitflag-attr/commit/67c5cded79a683bde30e9b274a6553adaddf1fc9))
- Add proper documentation for the crate features. — ([68d2f5b](https://github.com/GrayJack/bitflag-attr/commit/68d2f5b6d352cab3613615bef7ad272b5a203dab))
- Modify the changelog format a bit to avoid rustdoc `unresolved link` issues — ([f418c2b](https://github.com/GrayJack/bitflag-attr/commit/f418c2b491b58a8f246ff62816724aa6b140df88))
- Do not run doctest on the spec file — ([4ab8cc4](https://github.com/GrayJack/bitflag-attr/commit/4ab8cc4f7f200a22d9f2862410e5ba4329f6c7e8))
- Update syntax section of `bitflag` macro and fix some typos — ([87c021d](https://github.com/GrayJack/bitflag-attr/commit/87c021d2189b0689798720c5056022da8588fdda))
- Document the custom `Debug` implementation on the `bitflag` macro — ([4e094f0](https://github.com/GrayJack/bitflag-attr/commit/4e094f05c072f8f83f211c77e9b33842ecfd41ec))
- Clarify internal item — ([17a6fb8](https://github.com/GrayJack/bitflag-attr/commit/17a6fb8f7b426ed067e73e1d2d48f57e29b351e0))
- Expand the crate entry documentation — ([7958b99](https://github.com/GrayJack/bitflag-attr/commit/7958b99371e386a5f0536c985cdfab666771251f))
- Expand parser module documentation — ([773be5a](https://github.com/GrayJack/bitflag-attr/commit/773be5a168ba6707524c197ded9f98a73a3c7e43))
- Clarify some parts — ([3b2eac6](https://github.com/GrayJack/bitflag-attr/commit/3b2eac6dd3a9a56fe5fac2eea26ce98ab0cda0b6))
- Add example without using the `bitflag` macro — ([561a362](https://github.com/GrayJack/bitflag-attr/commit/561a362478e35cb89822593d3180204164807c9c))
- Re-organize some example code — ([189a86b](https://github.com/GrayJack/bitflag-attr/commit/189a86b43416c00fa1d39352d56e96ce7afc4b3e))
- Add a example for deriving Default — ([f566240](https://github.com/GrayJack/bitflag-attr/commit/f566240dc5fc592edbdc135d4eac82822ee4a6cc))
- Example for external flags — ([777caf6](https://github.com/GrayJack/bitflag-attr/commit/777caf6c4dfffc4bd907dd03b9d81da75aed9895))
- Small tweaks — ([fdf7562](https://github.com/GrayJack/bitflag-attr/commit/fdf75622b6d76b112ef116b755df57ab514c9bc2))
- Better naming on examples — ([d6114b3](https://github.com/GrayJack/bitflag-attr/commit/d6114b3f8a9ffe35026ca91830262aa56a3513ec))

## [0.11.1](https://github.com/GrayJack/bitflag-attr/compare/v0.11.0..v0.11.1) — 2025-02-15

### 🚀 Features

- Improve error on using the attribute on something that is not a enum — ([6521cd9](https://github.com/GrayJack/bitflag-attr/commit/6521cd976c5a32c113af6727277b810aa498639a))
- Add method to create a value with all the named flags. — ([d029a1d](https://github.com/GrayJack/bitflag-attr/commit/d029a1d8c7498271f6322b02199e85ba1db4bfed))
- Generate `contains_unnamed_bits` method — ([5eddbf6](https://github.com/GrayJack/bitflag-attr/commit/5eddbf6cc56d96b2fda11aed65e210a3903c63bc))

### 🐞 Bug Fixes

- Fix compilation issue on CI — ([b59475d](https://github.com/GrayJack/bitflag-attr/commit/b59475d3c4d131f6059619e34f70ef3b06426702))

## [0.11.0](https://github.com/GrayJack/bitflag-attr/compare/v0.10.0..v0.11.0) — 2025-02-14

### 🚀 Features

- Handle full path of custom external derives — ([708097e](https://github.com/GrayJack/bitflag-attr/commit/708097eef7df4396f2aacbd906804cb7189a1405))
- Accept more well known types and full-path — ([47b3dca](https://github.com/GrayJack/bitflag-attr/commit/47b3dca6eee898f27f931e3812f366ff3095c8ae))
- Add inline on some generated functions — ([3c1d446](https://github.com/GrayJack/bitflag-attr/commit/3c1d44651871021779d841fa58a3823201009957))
- (**BREAKING**) Debug outputs octal and hex as well — ([c72b604](https://github.com/GrayJack/bitflag-attr/commit/c72b604bb6080fe818f65f72535eaac584f783b1))
- Add method `clear` for the generated type and the `Flags` trait — ([0e20342](https://github.com/GrayJack/bitflag-attr/commit/0e20342cdc763949f9165c73027216dfd87d1482))

### 🐞 Bug Fixes

- Process static assert error string in the macro — ([051fe86](https://github.com/GrayJack/bitflag-attr/commit/051fe86b7efe71a5eea79c6537669561068c5169))

### 🚜 Refactor

- Inline small function — ([728630a](https://github.com/GrayJack/bitflag-attr/commit/728630ac6dfbca26c63f4b5f0189e85b0847aabf))
- Simplify code generation a bit — ([7b5e02a](https://github.com/GrayJack/bitflag-attr/commit/7b5e02ab910860e53694a6eb6ac48c25349866e4))

### 📚 Documentation

- Fix links — ([e67f383](https://github.com/GrayJack/bitflag-attr/commit/e67f383377fc8547fdc42869634c49f855e7652d))
- Fix lint — ([9b2b185](https://github.com/GrayJack/bitflag-attr/commit/9b2b1858bf3b2ede74c7179b6e7f90c7a5ec4650))
- Update readme — ([c4959eb](https://github.com/GrayJack/bitflag-attr/commit/c4959ebf39f366a18645c50ce61e66f804812620))
- Update README — ([a4eb9b7](https://github.com/GrayJack/bitflag-attr/commit/a4eb9b756aa13a9f5c8da8d730ae8bb1c59aa604))
- Fix typos — ([15da0fb](https://github.com/GrayJack/bitflag-attr/commit/15da0fb9364cb36a8ba0c9b64ff2d23480ba002c))
- Add some doc alias — ([1e16e82](https://github.com/GrayJack/bitflag-attr/commit/1e16e8272435dc931df47228d026b18f9a1ef398))
- Fix missing links to `bitflags` macro docs — ([40387f9](https://github.com/GrayJack/bitflag-attr/commit/40387f9e00c29a62dd9bebb96eb8526467d5446d))
- Document newly accepted types in the bitflag attribute — ([c79c418](https://github.com/GrayJack/bitflag-attr/commit/c79c4183902904dfc3a644c5427d320a989a4332))

### ⚙️ Continuous Integration

- Check all features — ([5f77b41](https://github.com/GrayJack/bitflag-attr/commit/5f77b412c8209891ba22995ff61937f706f8a592))
- Fix testing toolchain versions — ([0dea689](https://github.com/GrayJack/bitflag-attr/commit/0dea6899d2c65f03723f654d7841f39849aa5dcc))
- Update docs flags — ([ccdfe4f](https://github.com/GrayJack/bitflag-attr/commit/ccdfe4fd299256336c8b253e598a67ff131b1bd2))

## [0.10.0](https://github.com/GrayJack/bitflag-attr/compare/v0.9.0..v0.10.0) — 2025-02-14

### 🚀 Features

- (**BREAKING**) Specify all traits we use on generated code on `BitsPrimitive — ([387e82d](https://github.com/GrayJack/bitflag-attr/commit/387e82d4c59885f96ed0dd00316e3f9c53ccd907))
- Handle explicit `repr` on type definition — ([aec8085](https://github.com/GrayJack/bitflag-attr/commit/aec8085cb0172becc007433d160da51098e2d1e6))
- (**BREAKING**) Handle `derive(Default)` on `bitflag` macro — ([cb0ee4e](https://github.com/GrayJack/bitflag-attr/commit/cb0ee4edbfdf3f88db26fb6ac06806b13342dccb))
- Error on `#[serde]` helper attributes — ([61c217f](https://github.com/GrayJack/bitflag-attr/commit/61c217fbfb41319268b120a52cb2647677c93082))
- Implement custom derive for `arbitrary::Arbitrary` for types using `bitflag` attribute macro — ([54113c4](https://github.com/GrayJack/bitflag-attr/commit/54113c4c9a05333dc37fa46ae353a31a64e8bcaa))
- Simplify more paths — ([0ba0510](https://github.com/GrayJack/bitflag-attr/commit/0ba05106e95c33235f9f325654d04898d151acc8))
- Add `bytemuck` support — ([3274172](https://github.com/GrayJack/bitflag-attr/commit/327417271be916c8939d7270eb169e197cbd493e))
- Implement complete parsing for the `repr` attribute — ([cd67362](https://github.com/GrayJack/bitflag-attr/commit/cd6736277180afb5656c36322f8122a7cc418535))
- Simplify — ([1c1437e](https://github.com/GrayJack/bitflag-attr/commit/1c1437e73bfd58338365e8b882f0b75a45a5e1e3))

### 🐞 Bug Fixes

- Fix extra `;` generated — ([9334972](https://github.com/GrayJack/bitflag-attr/commit/9334972c6af1ff8303f7cc9ea34c0444ebbb43ca))
- Fix check — ([4f4fa48](https://github.com/GrayJack/bitflag-attr/commit/4f4fa4861774b0f312be5c36a25880f608d83b02))
- Pass external custom derives to the struct when without feature enabled — ([64e8263](https://github.com/GrayJack/bitflag-attr/commit/64e82638f9baeeb6d8b66ee1e3ef9c9017a1a133))
- Use full namespace for `Option`and `Result` on generated code — ([7df850d](https://github.com/GrayJack/bitflag-attr/commit/7df850dcd06b6d1c9c2d22601a1f7835145dec01))

### 🚜 Refactor

- Simplify — ([371ec79](https://github.com/GrayJack/bitflag-attr/commit/371ec79e916b82d72b1f7341ac190d2054d526e4))
- Clean up — ([f9cc9c5](https://github.com/GrayJack/bitflag-attr/commit/f9cc9c5ebb6096207642082e8657cfa335384b91))

### 📚 Documentation

- Fix typo — ([381ae8e](https://github.com/GrayJack/bitflag-attr/commit/381ae8ecce213370a247a4a07cccaed620be9ce5))
- Update generated, fix links — ([9c14e15](https://github.com/GrayJack/bitflag-attr/commit/9c14e158b88056973918ba1fd75142cfba032845))

### 🧪 Testing

- Another compilation error rest annoying difference of span reporting between stable and nightly — ([fbb3d70](https://github.com/GrayJack/bitflag-attr/commit/fbb3d707f7f50d6515d152b8ac553b6e5a935ba7))
- Add better tests for serde feature — ([2c57bd5](https://github.com/GrayJack/bitflag-attr/commit/2c57bd5ff4f6a149fd2babb818e0b3c9b934b37f))

## [0.9.0](https://github.com/GrayJack/bitflag-attr/compare/v0.8.2..v0.9.0) — 2025-02-14

### 🚀 Features

- Add ability to modify the all possible/valid bits. — ([e0ad46a](https://github.com/GrayJack/bitflag-attr/commit/e0ad46a3f7ec414708537471a8ee9cad56d3eed3))
- Add way to configure the extra valid bits for externally defined flags — ([faec7f4](https://github.com/GrayJack/bitflag-attr/commit/faec7f4eb261b3a1cc24c3130e65f423d75e99b7))
- (**BREAKING**) Move iterators and parsing from macro generated code to generic code in the `bitflag_attr` crate — ([b1b82b9](https://github.com/GrayJack/bitflag-attr/commit/b1b82b9272137c23448f408cb31e107f8e25e2d8))
- (**BREAKING**) Make debug bits representation always show all bits — ([57db5d4](https://github.com/GrayJack/bitflag-attr/commit/57db5d44e55334f5c9cbb9c08091daba90180d5b))
- More fine grained with `alloc` crate feature — ([d912abc](https://github.com/GrayJack/bitflag-attr/commit/d912abc0112819f06194ea5ae317812a10280fe0))
- Implement the `bitflag_match!` macro — ([6ee32fd](https://github.com/GrayJack/bitflag-attr/commit/6ee32fd472a21e35360d66eae7e6b461de5218aa))

### 🐞 Bug Fixes

- Fix type name on doc string — ([1cc2567](https://github.com/GrayJack/bitflag-attr/commit/1cc2567aff90144ff07f450d2f44384e8b67564e))
- Fix debug impl not outputing text of human readable field when flags are empty — ([a28372d](https://github.com/GrayJack/bitflag-attr/commit/a28372d85df66ae36e30ca10c6de4ebc25e6a4ee))

### 🚜 Refactor

- Refactor and clean the macro code — ([582dd24](https://github.com/GrayJack/bitflag-attr/commit/582dd24c0fb0d3cec13dc2150da7903bb76f7029))
- Make `truncate` generated method modify the value — ([3f9461b](https://github.com/GrayJack/bitflag-attr/commit/3f9461b21cf7e50ad71eaab15fc31de04c98af29))
- Rename one of debug representations from `human_readable` to `flags` — ([ea0f932](https://github.com/GrayJack/bitflag-attr/commit/ea0f932cbfe6cea3e69810b259bfa5894c36f49f))

### 📚 Documentation

- Update docs examples — ([421e26b](https://github.com/GrayJack/bitflag-attr/commit/421e26ba521b8beb4e047828ebc8800803895f5b))
- Add crate level docs based on bitflags — ([666267e](https://github.com/GrayJack/bitflag-attr/commit/666267ec415a44f8ebaef002325bb589a4e27218))
- Update and expand documentation — ([fb21d79](https://github.com/GrayJack/bitflag-attr/commit/fb21d7949c2bd46774c10b6f4b42bb939226b328))

### ⚡ Performance

- Remove unneeded cloning — ([889090f](https://github.com/GrayJack/bitflag-attr/commit/889090f8de1433f9495d195a47784bb5d0071eb1))

### 🧪 Testing

- Import tests from the bitflags crate — ([b9b7e79](https://github.com/GrayJack/bitflag-attr/commit/b9b7e791ebbedb0dbee735294fb082d2e1ea44c7))

## [0.8.2](https://github.com/GrayJack/bitflag-attr/compare/v0.8.1..v0.8.2) — 2025-02-14

### 🚀 Features

- Greatly reduce the amount of code generated — ([bec2df6](https://github.com/GrayJack/bitflag-attr/commit/bec2df6a123159e1c88af320061f58637753536d))
- Generate constants and methods in different impl blocks — ([3cc9760](https://github.com/GrayJack/bitflag-attr/commit/3cc97600709eaee0b1f28360b19754e66080dcfb))

### 🚜 Refactor

- Simplify — ([0ecaa69](https://github.com/GrayJack/bitflag-attr/commit/0ecaa69f5e241edf92187a2edacf19959f1d4f78))

### 📚 Documentation

- Update generated example — ([86907cc](https://github.com/GrayJack/bitflag-attr/commit/86907cc38bb5e5fbab284e9b4ab5265153f7adb8))

## [0.8.1](https://github.com/GrayJack/bitflag-attr/compare/v0.8.0..v0.8.1) — 2025-02-14

### 🐞 Bug Fixes

- Help semantic highlight a bit more — ([4990bf3](https://github.com/GrayJack/bitflag-attr/commit/4990bf3bb815a90132f5879aff40e061ecbcc540))

## [0.8.0](https://github.com/GrayJack/bitflag-attr/compare/v0.7.4..v0.8.0) — 2025-02-14

### 🚀 Features

- (**BREAKING**) Use the type definition `#[define(...)]` arguments to check for required derives, and to decide if implement custom ones — ([aeb82a9](https://github.com/GrayJack/bitflag-attr/commit/aeb82a9f6249483ed23b2b1b3c6219a559be1747))

### 💼 Other

- Add `clone-impls` feature of `syn` — ([3c1790e](https://github.com/GrayJack/bitflag-attr/commit/3c1790e190613134345c0e5d4295f3b8d2c33423))

### 📚 Documentation

- Update docs with the previous commit changes — ([6187b9f](https://github.com/GrayJack/bitflag-attr/commit/6187b9f336a78383a36ecdef545c7e0ccbe55e7d))
- Update expanded example — ([18a961a](https://github.com/GrayJack/bitflag-attr/commit/18a961aeca122ed7b67cfaccea21be13659b55c6))

## [0.7.4](https://github.com/GrayJack/bitflag-attr/compare/v0.7.3..v0.7.4) — 2025-02-14

### 🐞 Bug Fixes

- Fix inserting documentation attributes where it shouldn't — ([664fae3](https://github.com/GrayJack/bitflag-attr/commit/664fae33f83e8300f4e5184d22796b98b8952505))

## [0.7.3](https://github.com/GrayJack/bitflag-attr/compare/v0.7.2..v0.7.3) — 2025-02-14

### 🐞 Bug Fixes

- Preserve attributes on all variants repetition — ([96016d1](https://github.com/GrayJack/bitflag-attr/commit/96016d1e34874c31c7ee34058ed89b7852ab5122))

### ⚙️ Continuous Integration

- Fix typo — ([525b180](https://github.com/GrayJack/bitflag-attr/commit/525b180426f0c48fef162e50d45966950abefb03))
- Enable miri checks for some tests — ([23a282b](https://github.com/GrayJack/bitflag-attr/commit/23a282b3a8bc15c530fbb42412b92758c83a58d2))

## [0.7.2](https://github.com/GrayJack/bitflag-attr/compare/v0.7.1..v0.7.2) — 2025-02-14

### 🐞 Bug Fixes

- Properly handle no_std — ([01eeb39](https://github.com/GrayJack/bitflag-attr/commit/01eeb39164916e29b596aaf996b47bced3b847f3))

## [0.7.1](https://github.com/GrayJack/bitflag-attr/compare/v0.7.0..v0.7.1) — 2025-02-14

### 📚 Documentation

- Document `BitflagPrimitive` trait — ([18301d1](https://github.com/GrayJack/bitflag-attr/commit/18301d1c5d0c409b397022fdc182da450b00c231))
- Update generated example for docs — ([143cdd5](https://github.com/GrayJack/bitflag-attr/commit/143cdd53e5b57df2db0e842e51f3ba1d28677080))

## [0.7.0](https://github.com/GrayJack/bitflag-attr/compare/v0.6.0..v0.7.0) — 2025-02-14

### 🚀 Features

- Make `set`, `unset` and `toggle` const — ([217cf58](https://github.com/GrayJack/bitflag-attr/commit/217cf58b0a153a4c05fcddebe304b7e415fe5317))
- Move the attribute macro to crate specific to proc-macro — ([dab23fb](https://github.com/GrayJack/bitflag-attr/commit/dab23fbfac46fe81dfa6ff34a8c669655e7ec0a8))
- Add support for custom types — ([7ad35b6](https://github.com/GrayJack/bitflag-attr/commit/7ad35b6a0aae0754f85ca4846202e887ab028b20))
- Make generating const function with mutable reference behind a feature flag — ([28aeb1b](https://github.com/GrayJack/bitflag-attr/commit/28aeb1bfb655d2e374c0b05f14100d02a1b47452))

### 🧪 Testing

- Add test for no_std — ([0f40255](https://github.com/GrayJack/bitflag-attr/commit/0f402552e578713f7a1589abf6cdabf2e354dc02))
- Add some tests for generated functionality — ([bb3c5cc](https://github.com/GrayJack/bitflag-attr/commit/bb3c5cccf5e4a4d5a7ce9f54aba503b24de6c88b))

## [0.6.0](https://github.com/GrayJack/bitflag-attr/compare/v0.5.0..v0.6.0) — 2025-02-14

### 🚀 Features

- Make sure trait implementation uses `core` traits — ([74f84cd](https://github.com/GrayJack/bitflag-attr/commit/74f84cd6127bc890b7b4df7b5057798b9c6c2cd0))
- Allow 128 bit integers as types — ([c7969c5](https://github.com/GrayJack/bitflag-attr/commit/c7969c50cd9df0a2077fa24794d946dae5d6dfb5))
- Implement generating iterators — ([0553788](https://github.com/GrayJack/bitflag-attr/commit/0553788ef5d7c6249fd098ca1447b7587337b2ed))
- Add helpers for formatting — ([6ac7b14](https://github.com/GrayJack/bitflag-attr/commit/6ac7b14e1f81e5a0c9466a0cb3314f06c2b1be7c))
- Implement `serde` feature — ([70a8de0](https://github.com/GrayJack/bitflag-attr/commit/70a8de049b71f4d94748c323c651d02131f23f91))

### 🐞 Bug Fixes

- Improve multi-bit flags on Debug impl — ([14865d2](https://github.com/GrayJack/bitflag-attr/commit/14865d2a441bd12900f397e897fc25c109fd17f6))
- Fix generated public API visibility — ([c1a5f0b](https://github.com/GrayJack/bitflag-attr/commit/c1a5f0bbe41bc5eccfb8c0b7281e6a71d563ff31))
- Fix code generation errors — ([0032e12](https://github.com/GrayJack/bitflag-attr/commit/0032e1252f1020616ada1cbb143ead34651fa5b5))
- `set` was doing an AND operation instead of OR — ([b31dac7](https://github.com/GrayJack/bitflag-attr/commit/b31dac7b311c5784090ca770d4c41f3f50358882))
- Fix code generation messing up semantic coloring of LSP — ([9019278](https://github.com/GrayJack/bitflag-attr/commit/901927892df36ecd6d3613bf671637fb4c9046f5))

### 🚜 Refactor

- Always implement the iterators — ([de52cd8](https://github.com/GrayJack/bitflag-attr/commit/de52cd887b1a3720354f984829ac4ca432baf757))
- Refactor Debug impl generation — ([a81b43f](https://github.com/GrayJack/bitflag-attr/commit/a81b43f7429eb65c1a8fbffa0125dff8c90ecf59))

### 📚 Documentation

- Clarify — ([5f09260](https://github.com/GrayJack/bitflag-attr/commit/5f09260b4d7e75c3ec3f881dfb20e48ab9a536d5))
- Fix typos; clarify a few points — ([6e36418](https://github.com/GrayJack/bitflag-attr/commit/6e36418026cb40b47878f93ec02e133afbd25c57))
- Add doc-alias to `set` and `unset` — ([b88a75e](https://github.com/GrayJack/bitflag-attr/commit/b88a75e35f0e5fb3dd6c7ef92fa7d06514d385c0))
- Update docs — ([0cda2e4](https://github.com/GrayJack/bitflag-attr/commit/0cda2e4c2d8a0c82e34cb98aa9f615e5b23b2b6e))

## [0.5.0](https://github.com/GrayJack/bitflag-attr/compare/v0.4.0..v0.5.0) — 2025-02-14

### 🚀 Features

- Add `#[automatically_derived]` to generated `impl Trait` blocks — ([242abac](https://github.com/GrayJack/bitflag-attr/commit/242abacc03ea417d2b0c36523377dc76e5cb8e10))
- Add option to not generate the `Debug` impl — ([a3059e7](https://github.com/GrayJack/bitflag-attr/commit/a3059e7e43e3b27bd989026ad86a0c76a556a7e7))
- Improve error message on parameters being repeated — ([9fd17d8](https://github.com/GrayJack/bitflag-attr/commit/9fd17d8a87cf259233a91a880c60dca6c362715a))

## [0.4.0](https://github.com/GrayJack/bitflag-attr/compare/v0.3.1..v0.4.0) — 2025-02-14

### 🚀 Features

- (**BREAKING**) Do not generate the the raw constants outside — ([8d48b33](https://github.com/GrayJack/bitflag-attr/commit/8d48b33d0d6594ed339ee300ce77fea025ef97f8))

## [0.3.1](https://github.com/GrayJack/bitflag-attr/compare/v0.3.0..v0.3.1) — 2024-10-19

### 🏗️ Build Logic

- Enable only the features that we need — ([f530073](https://github.com/GrayJack/bitflag-attr/commit/f530073435df5e14bb284f75c427ba1eb5947397))

## [0.3.0](https://github.com/GrayJack/bitflag-attr/compare/v0.2.0..v0.3.0) — 2024-05-16

### 🚀 Features

- Improve Debug  implementation — ([abacd20](https://github.com/GrayJack/bitflag-attr/commit/abacd20e68ad42b99ae7dd17cfceddf34ad54291))

### 🚜 Refactor

- Simplify generated Debug impl — ([0b2a399](https://github.com/GrayJack/bitflag-attr/commit/0b2a3998e9325486ddc03fe92e0164145a98c8d8))

## [0.2.0](https://github.com/GrayJack/bitflag-attr/compare/v0.1.0..v0.2.0) — 2024-05-15

### 🚀 Features

- Auto-derive `Clone` and `Copy` traits — ([22c20b4](https://github.com/GrayJack/bitflag-attr/commit/22c20b4a276ad822d1af36f8aedcb0ff66645783))

### ⚙️ Continuous Integration

- Remove disnecessary stuff — ([9de2ae5](https://github.com/GrayJack/bitflag-attr/commit/9de2ae54be9c274bea4d0d5c32f8e36d75787c00))
- Update lowest rust version — ([f149a55](https://github.com/GrayJack/bitflag-attr/commit/f149a55963a1897fa107864948324aa3d260f1dc))

## 0.1.0 — 2024-05-15

### 🚀 Features

- Initial version — ([ed4d623](https://github.com/GrayJack/bitflag-attr/commit/ed4d6231ec641714cbb4202307544b656be0aeee))

<!-- generated by git-cliff -->
