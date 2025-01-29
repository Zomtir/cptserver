## Release v1.3.0 (29th Jan 2025)

- feat: Affiliation and organisation restructure
    - Split affiliation and organisation class files
    - Added a few from_row and sql_map functions to classes
    - Added organisation_info
- feat: Outsource team rights into team_info
- refactor: Merge user data into statistic_organisation
- refactor: Implement FromStr for enums
- refactor: Handled a bunch of clippy warnings
- test: Added the first integration test

## Release v1.2.0 (12th Jan 2025)

- feat: Split lib and package functionality to enable integration testing
    - refactor: Hand over conn rather than self-serve
    - refactor: Move CONFIG out of common, db and error
    - refactor: Move non-db logic from db to utils
    - feat: Internal cptserver library src/lib.rs
    - feat: Static config for a test database
    - feat: Initial integration test for database connection
    - fix: Reworked non-functional unit test at print_sql/prep_sql
- feat: Added file_url to licenses
- feat: Changed license number from u8 to String
- feat: Increased nickname length from 20 to 40
- refactor: Renamed db updates to db migrations
- fix: Missing default values of organisation rights
- chore: DB status and release package folder
- chore: Update DB schema to v2

## Release v1.1.0 (21st Dec 2024)

- feat: Added user bank accounts
- feat: Added user licenses
- feat: Added club image, disciplines and chairman and club_info API
- feat: Added club statistics for user presence
- feat: Added option to assign courses to clubs
- feat: Automatic db schema install and update
- feat: Include location at event statistics
- refactor: Minor chrono definition changes
- fix: Relativ paths do not work on production
- fix: Limit chars on filename path
- chore: Created a release shell script
- chore: Include build process in release script
- chore: Update rust and cargo to 1.83
- chore: Update db schema to 1
- chore: Readme and Cargo cleanups

## Release v1.0.0 (12th Sep 2024)

- license: Cosmetic changes to the license and waiver
- feat: SQL and API rewrite for event and course updates
    - Event leader, supporter, participant presence
    - Event leader, supporter, participant filter for access
    - Event leader, supporter, participant registrations
    - Event acceptance status
    - Event occurrence status
    - Course leader, supporter, participant team sieves
    - Make event owner a distinct meta role
    - Adapated event and course related statistics for new user presence types
- feat: Added Organisations and Affiliations
    - Added DB layout
    - Added structs
    - Added SQL calls
    - Added routes
- feat: Added Club statistic organisation
- feat: Added Event statistic organisation
- feat: DB updates for acceptance, occurrence, filters, sieves, access
- feat: Made configs customizable
- feat: Use OnceLock for POOL and CONFIG
- feat: Optionally serialize user fields
- feat: Add height and weight to user
- feat: Added id to club_stocks
- feat: Group stats into stripped json
- feat: Allow DB cascade delete for event references
- feat: Prevent event owner from demoting oneself
- feat: Admin event withdraw
- feat: Event credential route for admins
- feat: Event course API for owner
- feat: Filter possessions by item
- feat: Allow admin to filter terms by club
- feat: Event statistic for participant age/gender divisions
- feat: Item category list for regular users
- feat: Event item preparation
- feat: Personal possession list
- refactor: Renamed user birthlocation to user birth_location
- refactor: Renamed user birthday to user birth_date
- refactor: Moved all routes in to modules
- refactor: Move all db calls into module
- remove: Removed user data declaration
- chore: Cargo update
- chore: More consistent function names
- fix: Fixed typo in event SQL call
- fix: Fixed service event presence pool check
- fix: Do not autodecline on date overlap for admins
- fix: Allow change of occurrence
- fix: Fixed read/write permissions on routes
- fix: Wrong pool checks for presence
- fix: Streamlined club membership for future terms
- fix: Inconsistent itemcat API parameters
- fix: Handle Rust 1.81 compiler warnings

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