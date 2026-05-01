# Pilot Acquisition Strategy

This document outlines the plan for onboarding the first users of the `rag-starter` agent.

## 1. Acquisition Channel
- **Primary Channel**: GitHub Discussions (Category: "Pilots").
- **Secondary Channel**: Internal company Slack/Discord if applicable.
- **Goal**: Recruit 5-10 "Pilot Users" who are comfortable with CLI tools and Rust environments.

## 2. Pre-commit Mechanism
To ensure high-quality feedback, all pilot users must agree to the following "Pre-commit":
- **Commitment**: Spend at least 1 hour using the agent for real daily tasks.
- **Deliverable**: Fill out a simple feedback form (or post in Discussions) detailing:
    - 3 things that worked well.
    - 2 critical bugs or friction points.
    - 1 feature request.

## 3. Onboarding Flow
1.  **Invite**: Send the pilot invitation link to the target group.
2.  **Access**: Users clone the repo and set up their `.env`.
3.  **Tutorial**: Users follow the `docs/RUST_LEARNING_TRACK.md` to get their first model call working.
4.  **Task**: Users are asked to perform a specific task (e.g., "Ask the agent to summarize a diff of a recent project").
5.  **Feedback**: Users post their findings in the dedicated Discussion thread.

## 4. Success Metrics
- **Retention**: At least 50% of pilot users use the agent more than once.
- **Reliability**: 0 unhandled crashes during the 1-hour test period.
- **Utility**: At least 3 users report that the agent saved them time on a specific task.
