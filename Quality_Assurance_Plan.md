# Quality Assurance Plan

## Quality Assurance Strategy

-  Overview: Testing is based on whether intended requirements are met and resulting product is similar to the envisioned version
-  Testing Methodolgies: Main testing is usability testing. Beyond it, black box testing is used as the primary way of testing. Unit testing also used in specific situations.
-  Automated vs. Manual Testing: Entire system will be tested manually

## Quality Factors & Metrics

| Quality Factor  | Description                   | Measurement Metric                         |
|-----------------|-------------------------------|-------------------------------------------|
| Performance     | System response time          | Average response time (ms)                |
| Scalability     | Ease of extending the software with new features    | Level of adherence to scalable software design principle |
| Usability       | Ease of use for users         | User satisfaction score from surveys      |
| Maintainability | Ease of modifying the codebase| Code complexity score (e.g., Cyclomatic Complexity) |
## Test Plan
Test Cases:
1. Creating a New Save:
  - Steps
    - Run the program
    - Enter a seed value
  - Expected outcome: A new save file is created named after the seed value. Main screen is acessed.
2. Creating a Task:
  - Steps
    - Double click on a date on calender or,
    - Click "New Task" button
    - Edit the name and priority values.
    - Click "Save Task" button.
  - Expected outcome: A new task is created and can be seen on "My Tasks".
3. Reaccessing a Profile Using an Already Existing Seed Value
 - Steps
    - Rerun the program
    - Enter a prieviously existing seed value.
  - Expected outcome: Tasks that created and points gained in that save file can be seen.
4. Marking a Task as Completed
 - Steps
    - Click check button on a task
    - Click "Yes" button on confirmation popup
  - Expected outcome: Task is marked done and points are given acorrding to the priority.
5. Deleting a task
 - Steps
    - Click Cross button on a task
  - Expected outcome: Task is removed and no points gained.

6. Expired Tasks
 - Steps
    - Create a task in calender before the curennt time or,
    - Create a task in calender after the curennt time and wait until the due date
  - Expected outcome: There is a "Expired" text under the task's name.

Bug Tracking: Bugs will be reported under each test case if test cases failed. Each bugs' status will be traced using a table which will be created using the test cases.

Authors: SualpGdk, B4andAfter
