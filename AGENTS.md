# Development Guidelines for CV Project

You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant.
Dioxus 0.7 changes every api in dioxus.
Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone.

## Build & Development Commands

### Essential Commands

```bash
# Start development server (with environment variables)
export $(cat .env | xargs)
dx serve

# Build the project
dx build

# Format code
cargo fmt

# Run all tests
cargo test

# Run specific test file
cargo test --test auth_tests
cargo test --test component_tests
cargo test --test model_tests
cargo test --test route_tests
cargo test --test view_tests

# Run tests with verbose output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Pre-commit hooks (format, lint, test)
prek run
```

### Project Setup

```bash
chmod +x setup.sh
./setup.sh
```

## Code Style Guidelines

### File Organization

- **Main entry**: `src/main.rs` - Contains `main()` and `App` component
- **Core library**: `src/lib.rs` - Contains route definitions and module exports
- **Modules**: Organized in `src/` with `mod.rs` files for proper exports
- **Tests**: All unit tests in `tests/` directory (not `src/`)

### Import Style

```rust
// Standard imports first
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// Local imports second
use cv::models::AuthUser;
use views::{Home, Blog, Profile};
```

### Component Guidelines

- Components use `#[component]` macro
- Function names start with capital letter or contain underscore
- Props are owned values (String, Vec<T>) not references (&str, &[T])
- Props must implement `PartialEq` and `Clone`
- Use `ReadOnlySignal` for reactive props

```rust
#[component]
pub fn SocialLink(href: String, icon: String, label: String) -> Element {
    rsx! {
        a {
            href: "{href}",
            target: "_blank",
            class: "group flex items-center gap-2 text-blue-500 font-bold hover:text-white transition-colors text-lg",
            i { class: "fab {icon} text-xl" }
            "{label}"
        }
    }
}
```

### RSX Patterns

- Use loops over iterators when possible
- Conditional rendering with `if` statements
- Expressions wrapped in braces
- Assets referenced with `asset!()` macro

```rust
rsx! {
    div {
        class: "container",
        for i in 0..5 {
            div { "{i}" }
        }
        if condition {
            div { "Condition is true!" }
        }
        img {
            src: asset!("/assets/image.png"),
            alt: "An image",
        }
    }
}
```

### State Management

- Local state: `use_signal(|| initial_value)`
- Memoized values: `use_memo(move || expensive_calculation())`
- Context: `use_context_provider(|| signal)` and `use_context::<Signal<T>>()`

```rust
let mut count = use_signal(|| 0);
let doubled = use_memo(move || count() * 2);

// Writing to signal
*count.write() += 1;
count.with_mut(|c| *c += 1);
```

### Error Handling

- Use `Result<T, ServerFnError>` for server functions
- Proper error propagation with `map_err`
- Descriptive error messages

```rust
#[server]
pub async fn exchange_code_for_user(code: String) -> Result<AuthUser, ServerFnError> {
    let user_info = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch user info: {}", e)))?;

    Ok(AuthUser { email })
}
```

### Feature Flags

- Use conditional compilation for server-only code
- Proper feature organization in `Cargo.toml`

```rust
#[cfg(feature = "server")]
{
    // Server-only code
}
#[cfg(not(feature = "server"))]
{
    // Client-only fallback
}
```

### Models & Data Structures

- Use `#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]`
- Public fields for simple structs
- Proper serde integration

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthUser {
    pub email: String,
}
```

### Routing

- Single enum with `#[derive(Routable, Clone, PartialEq)]`
- Use `#[layout(Component)]` for shared layouts
- Dynamic segments with `:name` syntax

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    BlogPost { id: i32 },
}
```

## Testing Requirements

### Test Organization

- Test files in `tests/` directory
- Named with `_tests.rs` suffix
- Comprehensive edge case coverage
- Unit tests for all major components

### Running Tests

```bash
# All tests
cargo test

# Specific test file
cargo test --test auth_tests
```

## Pre-commit Hooks

Automatically runs:

1. `cargo fmt` - Code formatting
1. `cargo test` - All tests
1. Markdown formatting
1. YAML formatting

Manual run: `prek run`

## Environment Configuration

- Use `.env` file for local development
- Template available in `.env.template`
- Load with: `export $(cat .env | xargs)`
- Server-only variables accessed with `std::env::var`

## Dependencies

- **Dioxus 0.7.1** with router and fullstack features
- **reqwest** for HTTP requests
- **serde** for serialization
- **oauth2** for Google authentication (server feature)
- **rusqlite** for database (server feature)
- **web-sys** for browser APIs

## Asset Management

- Assets in `assets/` directory
- Referenced with `asset!("/assets/path")` macro
- CSS files in `assets/styling/`
- Images and icons in `assets/` root
