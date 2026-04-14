# Acceptance Test and Acceptance Criteria
Creating a New Save:
  - Steps
    - Run the program
    - Enter a seed value
  - Expected outcome: A new save file is created named after the seed value. Main screen is acessed.
  - Type: Functional
---
Reaccessing a Profile Using an Already Existing Seed Value:
 - Steps
    - Rerun the program
    - Enter a prieviously existing seed value.
  - Expected outcome: Tasks that created and points gained in that save file can be seen.
  - Non-functional part: During the rerunning, the app is ready for user input withing less than 15 seconds of launch command. This test will be conducted in normal OS conditions and the average of 3 trials will be used.
---
Creating a Task:
  - Steps
    - Double click on a date on calender or,
    - Click "New Task" button
    - Edit the name and priority values.
    - Click "Save Task" button.
  - Expected outcome: A new task is created and can be seen on "My Tasks".
  - Non-functional part: At least 20 tasks can be added to any calendar day.

---

 Deleting a task
 - Steps
    - Click Cross button on a task
  - Expected outcome: Task is removed and no points gained.
 - Type: Functional
