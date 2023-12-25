# Time Capsule CRUD App

This is a CRUD (Create, Read, Update, Delete) application for managing Time Capsules. Time Capsules are items with content that are locked until a specified unlock date.

## Table of Contents

- [Time Capsule CRUD App](#time-capsule-crud-app)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Functions](#functions)
    - [1. Get Time Capsule](#1-get-time-capsule)
    - [2. Add Time Capsule](#2-add-time-capsule)
    - [3. Update Time Capsule](#3-update-time-capsule)
    - [4. Delete Time Capsule](#4-delete-time-capsule)
    - [5. Update Unlock Date](#5-update-unlock-date)
    - [6. Get All Time Capsules](#6-get-all-time-capsules)
    - [7. Mark Time Capsule as Opened](#7-mark-time-capsule-as-opened)
    - [8. Get Total Number of Time Capsules](#8-get-total-number-of-time-capsules)
    - [9. Get Time Capsules Count Before Date](#9-get-time-capsules-count-before-date)
  - [Testing Hierarchy](#testing-hierarchy)

## Introduction

This CRUD application allows you to manage Time Capsules. Each Time Capsule has a unique ID, content, and an unlock date. You can perform various operations such as adding, updating, deleting, and retrieving Time Capsules.

## Functions

### 1. Get Time Capsule

Retrieve a Time Capsule by providing its ID.

### 2. Add Time Capsule

Add a new Time Capsule with the specified content and unlock date.

### 3. Update Time Capsule

Update the content and unlock date of an existing Time Capsule.

### 4. Delete Time Capsule

Delete a Time Capsule by providing its ID.

### 5. Update Unlock Date

Update the unlock date of an existing Time Capsule.

### 6. Get All Time Capsules

Retrieve a list of all Time Capsules.

### 7. Mark Time Capsule as Opened

Mark a Time Capsule as opened (additional actions can be performed here).

### 8. Get Total Number of Time Capsules

Retrieve the total number of Time Capsules.

### 9. Get Time Capsules Count Before Date

Retrieve the count of Time Capsules whose unlock date is before a specified date.

## Testing Hierarchy

1. **Get Time Capsule**
   - Use this function to retrieve details of a specific Time Capsule.

2. **Add Time Capsule**
   - Add a new Time Capsule with content and an unlock date.

3. **Update Time Capsule**
   - Modify the content and unlock date of an existing Time Capsule.

4. **Delete Time Capsule**
   - Delete an existing Time Capsule by providing its ID.

5. **Update Unlock Date**
   - Change the unlock date of an existing Time Capsule.

6. **Get All Time Capsules**
   - Retrieve a list of all Time Capsules.

7. **Mark Time Capsule as Opened**
   - Mark a Time Capsule as opened (additional actions can be performed here).

8. **Get Total Number of Time Capsules**
   - Retrieve the total number of Time Capsules.

9. **Get Time Capsules Count Before Date**
   - Retrieve the count of Time Capsules whose unlock date is before a specified date.
