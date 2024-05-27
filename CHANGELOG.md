## Release v0.9 (27th May 2024)

- feat: Inventory management
    - feat: Added items and item categories
    - feat: Added user item possessions
    - feat: Added club item stocks
    - feat: Added functions to loan, return, hand-out and hand-back items
- feat: Added course requirements
- feat: Event admin can edit course belonging
- feat: club statistics
    - Added club statistic for terms valid at a point in time
    - Added club statistic for users being members at a point in time
    - Added club statistic for users being in a team but not in the club
- feat: Initial super user
- feat: Add event user/owner registrations
- chore: Cargo update
- chore: Mark unused variable
- fix: Lower case server config file

## Release v0.8 (29th Apr 2024)

- Renamed slot to event
- Refactored class into event
- Remove slot API for regular user
- No longer exclude events from having a course
- Updated DB layout for courses and events
- Added course owner/participant summons/unsummons
- Added event owner/participant invites/uninvites
- Use Union of summons/invites/unsummons/uninvites for owner/participation pools
- Added API for regular user participation
- Added event bookmarks
- Improved Club and Location attributes
- Added Location routes
- Improved anon call consistency
- Added skill admin routes
- Added club admin routes
- Reworked team rights
- Added min/max to competence_summary
- Extended API to deliver min and max skill ranks
- Improved event_list date scope of SQL call
- Fixed team_edit SQl call
- Fixed team_list SQL call
- Added WebBool for custom behaviour of parameter presence
- Use WebDateTime for slot windows
- Fixed new mysql_common named param debug
- Cargo package update (mysql)

## Release v0.7 (15th Mar 2024)

- Added a WAIVER file
- LICENSE update
- README updates
- DB layout update
- Cargo update
- Re-Used `cargo fmt`
- Started to use `cargo clippy`
- Update Rust toolchain from 1.68 to 1.74
- Update Rocket from 0.5.0-rc3 to 0.5.0
- Fixed mysql_common dependency
- Add a SQL debug function
- Fixed a DB call with more than 12 params
- Made SQL named parameters lower case because upper case is not working with the mysql_common package
- Started migration to a new internal function syntax (/role/module_object_action)
- Split the common module into submodules
- Changed Date and DateTime serialization
- Changed WebDate and WebDateTime handling
- Implemented direct slot note edit and owner management via slot login
- Added slot_participant_pool, which only refers to course invites for now
- Added course_owner_pool
- Added course_statistic_participant
- Added course_statistic_owner
- Added course_statistic_class
- Added a user nickname
- Elevated nickname as user core attribute
- Improved E-Mail regex
- Improved slot filtering by course
- Added check course_moderator_true
- Added a SlotStatus enum
- Added location as slot list filter
- Allowed users/admins to change slot key
- Allow client side slot key generation
- Added slot notes
- Allowed editing slot notes for service login
- Added a few visibility/access tags for courses/users/slots: public, scrutable, active
- Added route /regular/slot_list
- Added route /owner/event_info
- Added route /admin/class_info
- Added route /admin/event_info, /admin/event_edit, /admin/event_edit_password, /admin/event_delete
- Added route /admin/event suspend
- Add owner role for event participant list/add/remove
- Improved course_edit and course_delete
- Renamed Branch to Skill
- Renamed Rankings to (User) Compentence
- Introduced course Requirements
- Introduced a club structure
- Changed terms to require a club
- Prepared structure for items and item categories
- Prepared structure for club inventories
- Prepared strcuture for user possessions
- Added a SQL debug function
- Fixed three term SQL calls
- Fixed course_available SQL call
- Fixed course_edit SQL call
- Fixed user creation SQL call
- Fixed slot participant SQL calls
- Fixed course_avaibility SQL call
- Fixed a typo in /regular/course_responsibility
- Fixed a typo in /regular/event_create
- Fixed a typo in /regular/course_availability
- Fixed two typos for slot_participant
- Fixed two typos for course_create
- Refactored /mod/course_responsibility to reuse db call course_list
- Split out course_requirements into a dedicated table
- Changed the slot role to the service role
- Changed the member role to the regular role
- Allow begin/end dates to be optional for terms
- Cleaned up the term routes and db calls

## Release v0.6 (5th May 2023)

- DB schema update
- Changed DB connection info
- Made more use of `cargo fmt`
- Cargo, package and dependency updates
- Fixed a mysql_common dependency
- Added an intermediate CptError
- Unified the API error with the CptError type to a crate::Error
- Added teams, team members and course team invites
- Implemented Course login for a limited duration range around the slots
- Added many more user details, such as nationality and association info
- Fixed SQL query for removing team member
- Added a systemd template (sptserver.service)
- Replaced the address struct by an address string user attribute
- Added a mediapermission user attribute
- Fixed missing salt during user creation
- Fixed some admin roles
- Fixed slot end snapping

## Release v0.5 (7th Feb 2023)

- Database layout update
- Cargo update
- Added a client side user salt
- Added server and database default config values
- Reworked CptServer config
- Implemented support for Rocket log level
- Replaced slot `access` by `public` attribute
- Allow user admins to edit user details
- Allow course admins to edit class owners
- Cleaned up events and classes
- Cleaned up user_passord_edit
- Cleaned up password validation
- Cleaned up location login
- Changed slot login from slot_id to slot_key
- Fixed creating new users
- Fixed optional user enabled attribute
- Configred the ARM linker for cross-compilation

## Release v0.4 (12th Jan 2023)

- Cargo updates
- Moved many DB calls into dedicated files
- Fixed ranking summary not picking up user_id
- Added user ranking_list and ranking_summary
- Added term functionality
- Added course admin functionality
- Changed course moderator syntax
- Fixed a course moderator SQL call
- Fixed a reservation list SQL call
- User right restructure
- Reused right struct for team
- Changed event admin nomenclature
- Added event owner permissions
- Implemented list/add/remove/delte slot owner functionality
- Reworked slot window handling
- Ignore Rocket.toml in git
- Removed DurationRound trait
- Implemented WebDate struct to pass a date in the URL

## Release v0.3 (18th Oct 2022)

- Updated packages and dependencies
- Added the DB layout to the project (Database.sql)
- Implemented custom responder everywhere
- Added team rights
- Outsourced common functions in a dedicated file

## Release v0.2 (23rd Jun 2022)

- Updated Rocket to v0.5-rc2
- Moved DB info into a config file
- Established a DB connection pool (unsafe)
- Added custom error headers
- Users can now be disabled/enabled

## Release v0.1 (17th Nov 2021)

First public release.

- Basic README
- Public Domain LICENSE
- Use Rocket v0.4 as API framework
- Most classes and routes for login, courses, rankings, reservations, teams and users
- Minor static config file