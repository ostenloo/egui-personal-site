# Private Blog Setup

This personal site now includes a password-protected private blog section.

## How It Works

### Accessing Private Blog
1. Navigate to `/private` in the browser
2. You'll be prompted for a password
3. Enter the correct password to access the private blog posts
4. Once authenticated, you can view all private blog posts

### Password
The password is currently set in `src/app.rs`:
```rust
const PRIVATE_BLOG_PASSWORD: &str = "your_secure_password_here";
```

**IMPORTANT**: Change this to a secure password before deploying!

### Creating Private Blog Posts

Use the same `new_post.sh` script, but save files to the `private_blog_posts/` directory instead:

```bash
./new_post.sh
# When prompted, enter the title
# The script will create the file in blog_posts/
# Move it to private_blog_posts/ manually:
mv blog_posts/2025-12-29-*.md private_blog_posts/
```

Or manually create files in `private_blog_posts/` with the format:
```markdown
---
title: "Your Private Post Title"
date: "2025-12-29T12:00:00-06:00"
---

Your private content here...
```

### File Naming Convention
Private blog posts follow the same naming convention as public posts:
```
YYYY-MM-DD-HHMMSS-slug.md
```

Example: `2025-12-29-120000-secret-thoughts.md`

### Security Notes
- Password is checked client-side (this is not meant for highly sensitive content)
- Authentication state is NOT persisted (you'll need to re-enter the password on page refresh)
- All private blog posts use the same password
- Private blog posts are embedded in the WASM binary (not loaded from a server)

### Routes
- Public blog: `/blog` and `/blog/{slug}`
- Private blog: `/private` and `/private/{slug}`

### Deployment
Before deploying to production:
1. Change `PRIVATE_BLOG_PASSWORD` to a strong password
2. Consider using environment variables for the password
3. Test the authentication flow thoroughly
