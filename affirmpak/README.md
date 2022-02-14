# The AffirmPak v0.1 Standard

### 1. What is an AffirmPak?
AffirmPaks are packages that allow users to add custom extensionns to Capybafirmations. Things such as themes,
custom text, and asset overrides.

Everything is UTF-8. No "but"s.

### 2. The File Format
An AffirmPak is composed of the following:
1. A Manifest (affirmpak.json)
2. An `src/` directory
3. Assets inside the `src/`
4. A file called `lib.json` that registers the files to the app.

### 3. The Manifest
A manifest consists of
1. The Authors (e.g. "Boaty McBoatFace <boatymcboatface@boats.com>")
    1. This is equivalent to the git config by default!
    2. It is a list of strings.
2. Name
    3. The name of the AffirmPak
3. Version
    1. SemVer-2 REQUIRED!
4. Compatibility Version Range
    1. Standard SemVer Notation
    2. Corresponds to the `internals` crate version.
5. Repository/Source Link
    1. Usually a git link, e.g. GitHub.
6. Description
    1. A short description of what this is.
7. Tags
    1. A maxiumum of 5 search words describing what this is
8. Documentation Link
    1. Optional
    2. Link to docs
9. Homepage Link
   1. Optional
10. Categories
    1. A list of catagories this fits into.
11. README link
    1. By default, does not need to be specified as we use the README of the repository.
    2. If specified, the file pointed to is the new README.

### 4. src/
The `src/` is where all the assets are.

#### 4.1 Custom Text Extensions
The text definiton JSON must only contain:
- A beginning `{}` (for valid JSON)
- A tag containing the sub-namespace for this group of text
    - It may be "_" to be in the global namespace for this pak.
- A tag containing a language code at the end. If omitted, it is assumed to be `en`.
- A tag containing a short text description.
- A list inside the sub-namespace of the text.

Text themselves is defined in a HTXML-like custom markup language (that gets compiled to HTML).
Text must be completely encased in a `<message></message>` body. Anything outside is discarded.
- `<m>`: An individual chat bubble to be spoken.
    - `wait`: Time to await in ms before the next message.
- `<i>`: Italics
- `<b>`: Bold
- `<c>`: Color of Text
    - `color`: Color of text, either an standard HTML color (e.g. red) or  hex code (must start with #)
- `<s>`: StrikeThrough
- `<u>`: Underline
- `<ic>`: Inline Code
- `<cd>`: Code Block
- `<q>`: Quote
- `<w>`: Wavy Text
- `<sh>`: Shakey Text
- `<br>`: Newline

#### 4.2 Custom Theme Extensions
Using the Theme Engine, users can define custom color schemes that are automatically compiled to CSS and loaded by
the ThemeManager at runtime.

It is a JSON file.
- "font": A link to a woff2 font.
- "colors":
    - "background": Background Color
    - "selected": Color for selected item
    - "accent": Color for things that need to be seperate e.g. Buttons
    - "text": Text color
    - "highlight": Text Highlight Color
    - "link": Link Color
    - "love": Color of Hearts
    - "like": Color of Stars
    - "visited": Visited Link Color
    - "disabled": Color of disabled Items
- "overrides": Text containing custom CSS rules (advanced users only)
- "animal": Overrides for the capybara itself
    - img: link to SVG for capybara.
    - center: Center positon override (in % of SVG)
    - mouth: Mouth position (in % of SVG)
- A tag containing a short text description.
- "types": A list of above things that are changed. (e.g. ["font", "color"])

Additionally exists formatting blocks to dynamically fill in user data. A sub-form is defined as `{parent:offspring}`
If the first character needs to be capitalized, add a `:C` after the offspring.
- `pronoun`
    - The user's pronouns. There are 5 forms to this.
    - `nominative`: The nominative form, e.g. "He", "She"
    - `accusative`: The accusative form, e.g. "Him", "Her"
    - `pronomial`: The pronomial form, e.g. "His", "Hers"
    - `predicative`: The predicative form, e.g. "His", "Her"
    - `reflexive`: The reflexive form, e.g. "Himself", "Herself"
- `username`
    - The username of the user.
- `birthday`
    - The birthday of the user.
- `usehours`
    - Hours since user signup.


### 5. lib.json
This is the equivalent of a `lib.rs` file. It contains everything to be linked into the final file.
- "text_registers": A list of text JSONs to register
- "scheme_registers": A list of schemes to register.

### 6. Compiler
The compiler traverses the `lib.json` and finds files it references recursively and links them to the final BSON file.

The BSON file is a collection of all the previous 