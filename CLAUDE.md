# Interactive Broker Trading App Context Engineering 

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.  It contains rules for Rust development and deployment 

## Project Awareness & Context
- Project contexts are located in `prps/` folder.
- At the start of a new conversation, read`/prps/projectgoal.md`to review project's architecture, style, and constraints.
- Examples are provided in the `examples/` folder.  These examples may be useful to better understand what I am trying to build

## Project Plan
- Using the project goal provided in `prps/projectgoal.md` create a project plan in `prps/` if it does not exist.
- If project plan exist, use this a roadmap for development and tracking progress.
- Update and modify project plan as needed.
- Show the update and ask for confirmation of the project plan before editing this file.

## Version Control
- Follow version control workflow as outline in the file `prps/verson_control.md` file.

## Tasks 
- Use the `tasks.md` file in the `prps/` to track the status of all the tasks that need to be done
- If the tasks.md file is not found, create the file in `prps/tasks.md`.  If it already exist, update and modify file as project progress.
- Show the update and ask for confirmation before editing this file.
- Add new tasks to the tasks.md file 
- Do not work on tasks in the tasks.md that have already been completed.  Do not repeat these tasks.
- **Mark completed tasks in `tasks.md`** immediately after finishing them.
- Add new sub-tasks or TODOs discovered during development to `tasks.md` under a “Discovered During Work” section.

## Ssub agents support
- Use software architect agent will build the define features, create tasks and track project progress
- Use rust software engineer agent to build the code
- Use test engineer to build test suites to test each features

## 🧠 AI Behavior Rules
### Principle 1: Brutal Honesty First
- Your primary directive is to reflect reality.  Avoid intellectual theater and wishful thinking.
- **No Mocks**: Never create mock data, placeholder functions or simulated responses when a real integration point can be tested.  Your code must be proven to work with the actual system.
- **No Theater**: If an integration fails, a library is incompatible, or a requirement is feasible, state it immediately and clearly.  Do not pretend with elaborate nonfunctional code.
- **Reality Check**: Before implementing, verify that the integration points, APIs, or library you need actually exist and are accessible.
- **Admit Ignorance**: If you do not understand how something works, your first step is to investigate through analysis and testing, or to ask for clarification. Do not guess.
### Principle 2: Test-Driven Development (TDD) is mandatory
- You will need to follow strict Test-Driven Development for all feature implementation.
1. **RED**: Write a concise failing test that defines a new feature or requirement.
2. **GREEN**: Write an absolute minimun amount of code necessary to make the test pass.
3. **REFACTOR**: Clean up and improve the code you just wrote, ensuring all tests remain green.  Never skip or reorder this cycle.

### Principle 3: Todos
- Keep your todos list short (less than 4 todos) during implementation, modifications, or updates. 
- Track the todos in the tasks.md file.
- If your todos list is longer than 4 items, break them into several parts under each phase.
- Stop implementation after each part so that developer can review.

### Principle 3: One feature at a time
- Focus exclusively on completing a single, well-defined feature before moving to the next.
* Definition of Done: A task or feature is completed is defined by the following:
- All tests are written and passing.
- The code is confirmed to work in the real target environment.
- Integration with the actual system is verified.
- Documentation has been updated
- **NO FEATURE CREEP**: Resist the urge to add "nice-to-have" functionalities until the current, core feature is 100% completed and verified.
- When you are adding a new feature such as a new method or function, stop to ask whether me permission to build feature.
- Please fully explain reason for the function as a comment

###  Principle 4: Break things internally
* Proactively find you flaw before they become a problem for the developer and user
- **FAIL FAST**: your code should fail immediately and loudly when its assumptions are violated
- **AGRESSIVE VALIDATION**: Check every input and validation point. Assume nothing.
- **LOUD ERRORS**: When something breaks, provide clear, descriptive error messages.
- **TEST EDGE CASES**: Deliberately attempt to break your code with edge cases, invalid inputs, and unexpected conditions.

### Principle 5: Optimized only after it works
- Functionality and correctness comes first. Performance feature should be address methodically.
- **MAKE IT WORK**: the first priority is functioning code that passes all tests.
- **MAKE IT RIGHT**: refactor the working code for clarity, maintainability, and adherence to best practices
- **MAKE IT FAST**: Only optimize after profiling reveals a real, measurable bottleneck. Never optimized based on assuptions.
### General Principles:
- **Never assume missing context. Ask questions if uncertain.**
- **Never hallucinate libraries or functions** – only use known, verified Python packages.
- **Always confirm file paths and module names** exist before referencing them in code or tests.
- **Never delete or overwrite existing code** unless explicitly instructed to or if part of a task from `task.md`.


### 📎 Coding Style & Conventions
- **To be determine

### Structure & Modularity
- **Never create a file longer than 500 lines of code.** If a file approaches this limit, refactor by splitting it into modules or helper files.
- **Organize code into clearly separated modules**, grouped by feature or responsibility.
- Never hardcode sensitive information - Always use .env files for API keys and configuration

### 📎 Modification Guideline
- When modifying code, always ... tbd 

### 📚 Documentation & Explainability
- **Update `README.md`** when new features are added, dependencies change, or setup steps are modified.
- **Comment non-obvious code** and ensure everything is understandable to a mid-level developer.
- When writing complex logic, **add an inline `# Reason:` comment** explaining the why, not just the what.

### Testing & Reliability
- **After updating any logic**, check whether existing unit tests need to be updated. If so, do it.
- Do not write test results or update README.md after testing.
- **Tests should live in a `/tests` folder** mirroring the main app structure.
  - Include at least:
    - 1 test for expected use
    - 1 edge case
    - 1 failure case

