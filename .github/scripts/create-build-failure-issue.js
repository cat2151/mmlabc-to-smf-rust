/**
 * Create a GitHub issue when a build fails
 * This script is designed to be used with actions/github-script@v7
 * 
 * Required context variables (passed from GitHub Actions):
 * - github: GitHub API client
 * - context: GitHub Actions context
 * - core: GitHub Actions core utilities
 * 
 * Environment variables used:
 * - GITHUB_SHA: The commit SHA
 * - GITHUB_SERVER_URL: The GitHub server URL
 * - GITHUB_REPOSITORY: The repository name (owner/repo)
 * - GITHUB_RUN_ID: The workflow run ID
 * - GITHUB_REF: The git ref
 */

module.exports = async ({github, context, core}) => {
  try {
    const sha = process.env.GITHUB_SHA;
    const shortSha = sha.substring(0, 7);
    const title = `Build failed (${shortSha})`;
    const runUrl = `${process.env.GITHUB_SERVER_URL}/${process.env.GITHUB_REPOSITORY}/actions/runs/${process.env.GITHUB_RUN_ID}`;
    const ref = process.env.GITHUB_REF;
    const body = [
      'Build failed in GitHub Actions.',
      '',
      `Branch/Ref: ${ref}`,
      `Commit: ${sha}`,
      '',
      'Please investigate the build errors and fix them.',
      '',
      `Workflow run: ${runUrl}`
    ].join('\n');

    const owner = context.repo.owner;
    const repo = context.repo.repo;

    const requiredLabels = ['build-failure', 'bug'];

    // Ensure required labels exist before using them
    for (const labelName of requiredLabels) {
      try {
        await github.rest.issues.getLabel({
          owner,
          repo,
          name: labelName,
        });
      } catch (error) {
        if (error.status === 404) {
          await github.rest.issues.createLabel({
            owner,
            repo,
            name: labelName,
            color: labelName === 'bug' ? 'd73a4a' : 'ffa500',
            description:
              labelName === 'bug'
                ? 'Something is not working'
                : 'Automatically created label for failed builds',
          });
        } else {
          throw error;
        }
      }
    }

    // Check if issue already exists for this commit
    const issues = await github.rest.issues.listForRepo({
      owner,
      repo,
      state: 'open',
      labels: ['build-failure']
    });

    const existingIssue = issues.data.find(issue =>
      issue.title === title
    );

    if (!existingIssue) {
      await github.rest.issues.create({
        owner,
        repo,
        title: title,
        body: body,
        labels: requiredLabels
      });
      core.info(`Created issue: ${title}`);
    } else {
      core.info(`Issue already exists for commit ${shortSha}`);
    }
  } catch (error) {
    core.error(`Failed to manage build failure issue: ${error.message ?? error}`);
    core.setFailed(error.message ?? String(error));
  }
};
