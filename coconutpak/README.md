# The CoconutPak v0.1.0 Standard

### 1. What is an coconutpak?
CoconutPaks are packages that allow users to add custom extensions to KindKapiBari. Things such as themes,
custom text, and asset overrides.

Everything is UTF-8. No "but"s. I don't care your 10 year old bullshit machine can't handle Unicode and can only do 7-bit signed ASCII.

### 2. The File Format
A coconut is composed of the following:
1. A Manifest (Coconut.toml)
2. An `src/` directory
3. Assets inside the `src/`
4. [Optional] README.md file that gives a long description of what this coconutpak does

### 3. The Manifest (Coconut.toml)
A manifest consists of
1. The Authors (e.g. "Boaty McBoatFace <xx_desTinyfan42069ipedojacketpeoplewhosavelivesandvictimblamerapevictims_xx@uwuvowshsohot.csis.gc.ca>")
    1. This is equivalent to the git config by default!
    2. It is a list of strings(for multiple authors).
2. Name
    1.The name of the coconutpak
    2. Can only contain A-Z, a-z, 0-9, -, _
3. Version
    1. SemVer-2 REQUIRED!
    2. 0.1.0 by default
4. Compatibility Version Range
    1. Standard SemVer Notation
    2. Corresponds to the `internals` crate version.
5. Repository/Source Link
    1. Usually a git link, e.g. GitHub.
6. Description
    1. A short description of what this is.
7. Readme:
   1. While this is included, the compiler will package any README.md file in the same working directory into this file.
8. Tags
    1. A maximum of 5 search words describing what this is
    2. Can only contain A-Z, a-z, 0-9, -, _
9. Documentation Link
    1. Optional
    2. Link to docs
10. Homepage Link
    1. Optional
11. Categories
    1. A list of categories this fits into.
    2. Can only contain A-Z, a-z, 0-9, -, _
12. README link
    1. By default, does not need to be specified as we use the README of the repository.
    2. If specified, the file pointed to is the new README.

### 4. src/
The `src/` is where all the assets are.

#### 4.1 Custom Text Extensions
The text definition HTML must only contain:
- A beginning `<CoconutPakText></CoconutPakText>` (root node)
- `<subnamespace>`: [STRING] A tag containing the sub-namespace for this group of text
    - Can be UTF-8
    - Can not contain " " or any other special character except for "_" and "-"
    - Can not start with a number or any other special character.
- `<tag>`: [STRING] The category of this subnamespace text extension. Something general, such as "voice"(if this is about voice) will suffice.
- `<langcode>`: [STRING] A tag containing a language code at the end. If omitted, it is assumed to be `en`(english).
  - ISO language codes. Check ISO-639 for more.
- `<description>`: [STRING] A tag containing a short text description.
- `<responses>`: A tag based markup similar to HTML/XML that defines each response

It must be in this specific order.
Text themselves is defined in an HTML-like custom markup language (that gets compiled to HTML).

Each response is rolled into a pool, and once a response is chosen it is rolled again to its individual probability. Both must succeed for 
a given message to appear. 

The files should end in `.copt` (**CO**conut**P**ak**T**ext) 

Note: The max time for responses are 5000 milliseconds of wait.

Note: Use markdown for basic text formatting. We follow the CommonMark standard, with strikethrough and tables enabled. 

Tags: 
- `<response>`: An individual response. There may be multiple.
  - `name`: This response's name. This is more lax
  - `probability`: A float from 0.0 to 1.0 saying the other roll chance of saying this message. (default to 1.0) 
  - `welcome`:A boolean indicating that this response should be in the startup message pool. (default false)
- `<message>`: An individual chat bubble to be spoken.
    - `wait`: Time to await in milliseconds before the next message. (default 500)
- `<color>`: Color of Text
    - `color`: Color of text, either a standard HTML color (e.g. red) or  hex code (must start with #)
- `<u>`: Underline
- `<super>`: Superscript
- `<sub>`: Subscript
- `<highlight>`: Highlight Text
- `<wave>`: Wavy Text
- `<shaky>`: Shaky Text
- `<br>`: Newline
- `<spoiler>`: Spoiler Text

You ***MUST*** close your tags. (Even `<br>`).  

##### Text Templating
Text templating is done using the [`tera`](https://crates.io/crates/tera).

Please refer to `tera`'s [documentation](https://tera.netlify.app/) for more.

We also have some helpful custom functions to help you:
- `genderreplace`:
  - Chooses the correct form based on the user's preferred gender.
  - Usage: `{{genderreplace(fem=[F], masc=[M], nb=[N], cs=[C])}}`
    - where:
      - [F]: Feminine Form (String)
      - [M]: Masculine Form (String)
      - [N]: Gender Neutral/Non-Binary Form (String)
      - [C]: Custom Form (String) (Note: if the value is ".", the custom gender will be returned as a string.)
- `datefmt`
  - Short for "Date Format", formats an availible date format in the requested format. If using a translatable date, it will automatically do so.
  - Usage: `{{datefmt(field=<D>,fmt=<F>)}}`
    - where:
      - <F>: Format string. Please see [`chrono`'s Documentation](https://github.com/chronotope/chrono#formatting-and-parsing) for more.
      - <D>: Date field, e.g. `"birthday"`.
- `efmt` 
  - Short for emoji fmt, Changes a emoji's skin tone based on user's settings.
  - Usage `{{efmt(<E>)}}`
    - where:
      - <E>: Emoji to format. This should be text, e.g. :open_hands: for 👐. 

Using formatters 
- `pronouns`:
  - The user's pronouns.
  - `nominative`: The nominative form, e.g. "He", "She"
  - `accusative`: The accusative form, e.g. "Him", "Her"
  - `pronomial`: The pronominal form, e.g. "His", "Hers"
  - `predicative`: The predicative form, e.g. "His", "Her"
  - `reflexive`: The reflexive form, e.g. "Himself", "Herself"
- `gender`:
  - The gender of the user. There are many possible values:
    - `"man"`
    - `"woman"`
    - `"non-binary"`
    - A Custom, user-defined, gender. This will be given to you in string form.
  - This is not meant to be used directly, instead to be used in a conditional
- `username`
  - The username of the user.
- `birthday`
  - The birthday of the user. 
  - This is a date field.
  - This will be empty if it does not exist. (Will return Jan 1 1970 since that is the start of time)
  - ***DO NOT*** use this directly! Use the `datefmt` helper instead.
- `registerdate`
  - The register date of the user.
  - This is a date field.
  - ***DO NOT*** use this directly! Use the `datefmt` helper instead.
- `langtag`
  - The entire BCP 47 language tag.
- `lang`
  - The preferred language of the user.

#### Example

```xml
<CoconutPakText>
    <subnamespace>turtle거북이</subnamespace>
    <tag>Pets(Turtle)</tag>
    <langcode>en</langcode>
    <description>Response about turtles.</description>
    <responses>
        <response name="i_love_your_turtle" probability="0.12">
            <message>I *love* your turtle!</message>
            <message wait="5000">They're <color color="#FF0000"><wave>just</wave></color> so <shaky>pretty!</shaky></message>
        </response>
        <response name="hi_username" welcome="true">
            <message>Turtle says "Hi {{username}}!"</message>
        </response>
    </responses>
</CoconutPakText>
```

#### 4.2 Custom Theme Extensions
Using the Theme Engine, users can define custom color schemes that are automatically compiled to CSS and loaded by
the ThemeManager at runtime.

It is a JSON file.
- "font": A link to a woff2 font.
- "colors":
    - "background": Background Color
    - "selected": Color for selected item
    - "accent": Color for things that need to be separate e.g. Buttons
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
    - center: Center position override (in % of SVG)
    - mouth: Mouth position (in % of SVG)
- A tag containing a short text description.
- "types": A list of above things that are changed. (e.g. ["font", "color"])

### 5. lib.json
This is the equivalent of a `lib.rs` file. It contains everything to be linked into the final file.
- "text_registers": A list of text JSONs to register
- "scheme_registers": A list of schemes to register.

### 6. Compiler
The compiler traverses the `lib.json` and finds files it references recursively and links them to the final BSON file.

### 7. Transfer Format
Since the registry must compile all coconutpaks itself, we must have a way to transfer files.
This is done using a tarball compressed with gzip. The only things there should be the `src/` directory and `Coconut.toml` file,
both at the root of the tarball.
