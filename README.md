# Taian's CV & Personal Hub

A real-time, dynamic CV and personal landing page built with [Dioxus](https://dioxuslabs.com/).

## Goal

The goal of this project is to create a professional and visually stunning CV page that serves as a central hub for all my professional links (LinkedIn, GitHub, etc.) and displays real-time status/stats from these platforms.

## Features

- **GitHub Integration**: Real-time display of user profile and repository stats.
- **Google OAuth2 Authentication**: Secure login to access profile features.
- **Personal Profile**: Display user-specific attributes and stats.
- **Modern Design**: Premium, dark-themed UI with glassmorphism and Tailwind CSS.
- **Local Persistence**: Independent session management using localStorage.

## Tech Stack

- **Framework**: [Dioxus](https://dioxuslabs.com/) (Rust)
- **Styling**: Tailwind CSS & Vanilla CSS
- **Database**: SQLite (Rusqlite) for caching
- **Auth**: Google OAuth2

## Quick Start

The easiest way to get started is to use the automated setup script:

```bash
chmod +x setup.sh
./setup.sh
```

This script will:

1. Install `cargo-binstall`.
1. Install `dioxus-cli`.
1. Install `prek`.
1. Activate Git hooks.
1. Create a `.env` file from the template.

## Configuration

1. **Set up Google OAuth2**:
   - Create a project at [Google Cloud Console](https://console.cloud.google.com/).
   - Create OAuth 2.0 Credentials (Web Application).
   - Add `http://localhost:8080/auth/callback` to the **Authorized redirect URIs**.
   - Copy the Client ID and Client Secret into your `.env` file.

## Running the Application

1. **Start the development server**:

   ```bash
   # Load environment variables and start
   export $(cat .env | xargs)
   dx serve
   ```

1. **Open in browser**:
   Navigate to `http://localhost:8080`

## Development Hooks

This project uses [prek](https://github.com/j178/prek) to manage [pre-commit](https://pre-commit.com/) hooks.

- **Manual Run**: `prek run`
- **Update Hooks**: `prek autoupdate`
