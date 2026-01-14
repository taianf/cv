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

## Configuration

1. **Copy the environment template**:
   ```bash
   cp .env.template .env
   ```

2. **Set up Google OAuth2**:
   - Create a project at [Google Cloud Console](https://console.cloud.google.com/).
   - Create OAuth 2.0 Credentials (Web Application).
   - Add `http://localhost:8080/auth/callback` to the **Authorized redirect URIs**.
   - Copy the Client ID and Client Secret into your `.env` file.

## Getting Started

1. **Install Dioxus CLI**:
   ```bash
   cargo install dioxus-cli
   ```

2. **Run the development server**:
   Make sure you have your environment variables set or use a tool to load them.
   ```bash
   # If using a bash-like shell:
   export $(cat .env | xargs)
   dx serve
   ```

3. **Open in browser**:
   Navigate to `http://localhost:8080`
