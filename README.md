!! In scratch phase, do not use until ver1.0.0 unless you want to help build it, Apis and features will change rapidly

# tails
a files for all your tales ;;

Planned Targets are: [Linux, Windows, macOS, Android, iOS and WASM]
> This is not compatible with no_std environments or classical web at the moment and isn't currently planned, not to say that wont be adapted later.

Concrete Goals:
.tails files should support the entire markdown standard but bring ergonomic expression alternatives, and extra 
tooling, this enables 0 migration friction with immediate unlock of new features using pulldown-cmark
.tails files should front matter variable injection to further keep individual files d.r.y. using greymatter
.tails files should be able to inject all other .tails and .md files inline so end users can have true single sources of truth
.tails files should return an actionable token array.
.tails should include dumb syntax tokenization for syntax highlighting using logos via simple common grammar matching.
.tails documents should be completely json serializable and deserializable
a cli file combinator tool so the standard is functional by itself without any gui or framework, that tool will be mit and a self-provided
insurance that your access CANNOT ever be taken away.

Potential Future Goals:
> supporting other langs, serde can turn the entire mother type into json so theoretically you just rebuild the ast in the
native language types.
> charts
> iframes

--at the current state, api's are completely in the air and may change multiple times a day-- when an api is becoming concrete, that api will be documented here--

Disclosure(Why): I do not believe that you are consenting to use an ecosystem if your data is not portable and in a format that you can use elsewhere easily. So I am going to make it a dependency and use the exact same api I am giving everyone else and I am going to make a cli 'aggregator' that lets you still benefit from the variable and file injection without any proprietary software.

Plus good notes and documentation just make the world a better place
