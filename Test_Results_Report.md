# Test Report
1. Test: Creating a New Save
   - Result: Pass
   - Test Results Overview: A new file was created
   - Test Results Analysis: This indicates a healthy save logic
---
2. Test: Reaccessing a Profile Using an Already Existing Seed Value
   - Result: Pass
   - Test Results Overview: Same profile was successfully accessed by the same seed after rerunning the app. The App launched within less than 15 seconds with average of 7.5 seconds in 3 trials.
   - Test Results Analysis: This indicates that the program can access and use the save file it created before and can be ready to use within a satisfactory amount of time.
---
3. Test: Creating a Task
   - Result: Pass
   - Test Results Overview: 20 tasks were successfully added to both current day and other calendar days
   - Test Results Analysis: While the task was successful, it was detected that while clicking the add task button, every 2nd click caused the "change name" setting to pop up as program mistook it for a double click. This potentially weakens user experience.
---
4. Test: Deleting a task
   - Result: Pass
   - Test Results Overview: Tasks were successfully deleted from both current day and other calendar days.
   - Test Results Analysis: This shows that task deletion functionality works as expected. It is important to note that double click issue mentioned in 3rd test also sometimes appeared in this one.

     
