import json
import os
from pathlib import Path

import requests


MAX_TOTAL_PATCH_CHARS = 120_000
MAX_FILES = 40
MODEL = os.getenv("OPENAI_MODEL", "gpt-5")
API_URL = "https://api.openai.com/v1/responses"


def load_json(path: str):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def compact_changed_files(files):
    compact = []
    used_chars = 0

    for f in files[:MAX_FILES]:
        patch = f.get("patch") or ""
        remaining = MAX_TOTAL_PATCH_CHARS - used_chars
        if remaining <= 0:
            break

        if len(patch) > remaining:
            patch = patch[:remaining] + "\n...[truncated]"

        used_chars += len(patch)

        compact.append(
            {
                "filename": f.get("filename", ""),
                "status": f.get("status", ""),
                "additions": f.get("additions", 0),
                "deletions": f.get("deletions", 0),
                "patch": patch,
            }
        )

    return compact


def build_instructions():
    return """You are a senior code reviewer with strong experience in Rust monorepos, multi-crate workspaces, backend gateway systems, authentication, billing, routing, and API integrations.

Review rules:
1. Only report issues supported by evidence in the diff.
2. Do not speculate about unseen code. If something is uncertain, label it as a hypothesis.
3. Prioritize:
   - correctness
   - security
   - error handling
   - edge cases
   - maintainability
   - missing tests
4. Distinguish clearly between blocking issues and non-blocking suggestions.
5. If no obvious blocking issue is found, say so clearly.
6. Keep the review concise, concrete, and diff-focused.
7. Reference filenames whenever possible.

Output must be English Markdown with this exact structure:

## Title
## Overall Assessment
## Blocking Issues
## Non-blocking Suggestions
## Suggested Tests
## Conclusion
"""


def build_input(repo_name, pr, files, comment_body, comment_author):
    payload = {
        "repository": repo_name,
        "pr_number": pr.get("number"),
        "title": pr.get("title", ""),
        "author": pr.get("user", {}).get("login", ""),
        "base_branch": pr.get("base", {}).get("ref", ""),
        "head_branch": pr.get("head", {}).get("ref", ""),
        "changed_files_count": pr.get("changed_files", 0),
        "trigger_comment_author": comment_author,
        "trigger_comment": comment_body,
        "pr_description": pr.get("body", ""),
        "changed_files": files,
    }

    return (
        "Review the following GitHub pull request diff.\n\n"
        "Return a practical PR review for maintainers.\n\n"
        f"{json.dumps(payload, ensure_ascii=False, indent=2)}"
    )


def call_openai(instructions: str, user_input: str) -> str:
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        raise RuntimeError("OPENAI_API_KEY is not set")

    response = requests.post(
        API_URL,
        headers={
            "Authorization": f"Bearer {api_key}",
            "Content-Type": "application/json",
        },
        json={
            "model": MODEL,
            "instructions": instructions,
            "input": user_input,
            "max_output_tokens": 4000,
        },
        timeout=180,
    )
    response.raise_for_status()

    data = response.json()

    if data.get("error"):
        raise RuntimeError(f"OpenAI API error: {data['error']}")

    output_text = data.get("output_text")
    if output_text:
        return output_text.strip()

    parts = []
    for item in data.get("output", []):
        for content in item.get("content", []):
            text = content.get("text")
            if isinstance(text, str) and text.strip():
                parts.append(text)

    return "\n".join(parts).strip()


def write_output(review: str):
    if not review:
        review = """## Title
GPT PR Review

## Overall Assessment
No usable review output was returned.

## Blocking Issues
- Unable to parse model output.

## Non-blocking Suggestions
- Check the OpenAI API response format and workflow logs.

## Suggested Tests
- Re-run the workflow on the same PR.
- Verify that pr.json and pr_files.json were generated correctly.

## Conclusion
The review did not complete successfully.
"""

    footer = """

---

Trigger: `@gpt review`  
Note: This review is generated from the PR diff and may not reflect code outside the visible changes.
"""
    Path("review-output.md").write_text(review + footer, encoding="utf-8")


def main():
    repo_name = os.environ.get("REPO_NAME", "")
    comment_body = os.environ.get("COMMENT_BODY", "")
    comment_author = os.environ.get("COMMENT_AUTHOR", "")

    pr = load_json("pr.json")
    pr_files = load_json("pr_files.json")

    compact_files = compact_changed_files(pr_files)
    instructions = build_instructions()
    user_input = build_input(
        repo_name=repo_name,
        pr=pr,
        files=compact_files,
        comment_body=comment_body,
        comment_author=comment_author,
    )

    try:
        review = call_openai(instructions, user_input)
    except Exception as e:
        review = f"""## Title
GPT PR Review

## Overall Assessment
The automated review failed before producing a normal result.

## Blocking Issues
- Workflow/runtime error: `{type(e).__name__}: {str(e)}`

## Non-blocking Suggestions
- Check repository secrets.
- Check OpenAI API access and model availability.
- Check the workflow logs for the request/response path.

## Suggested Tests
- Re-run the workflow.
- Verify `OPENAI_API_KEY`.
- Verify that `pr.json` and `pr_files.json` contain valid data.

## Conclusion
The review could not be completed due to an execution error.
"""

    write_output(review)


if __name__ == "__main__":
    main()