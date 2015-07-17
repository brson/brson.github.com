---
layout: basic
---

# 2015-07-16

* [Responded to pnacl port](https://github.com/rust-lang/rust/pull/26505#issuecomment-121976244)
* [Reviewed from_raw_parts assert](https://github.com/rust-lang/rust/pull/27068)
* [Asked glandium for Linux->Mac cross toolchain](https://bugzilla.mozilla.org/show_bug.cgi?id=1183850#c11)
* Responded to pings
* [Reviewed intofd patches](https://github.com/rust-lang/rust/pull/27064)
* Entered q3 goals in workday
* Updated windows ami with new cert
* Updated macs with new cert
* [Fixed a multirust override directory comparison bug](https://github.com/brson/multirust/commit/cfa900a77cb6d3c8bd1ad48ec5b33abd33e88e3f)
* [Reviewed llvm upgrade](https://github.com/rust-lang/rust/pull/27076)
* Sent prod meeting minutes
* [Started new twir](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/32?u=brson)
* [Replied to Q about mutating boxes](https://users.rust-lang.org/t/why-are-we-able-to-change-the-content-of-box/2125)

# 2015-07-15

* Emailed minutes from yesterday's support meeting
* [Reviewed Gankro's TARPL PR](https://github.com/rust-lang/rust/pull/27032#issuecomment-121679712)
* [Reviewed no_std RFC](https://github.com/rust-lang/rfcs/pull/1184#issuecomment-121732953)
* Sent prod meeting minutes
* Learned how to record vidyo calls
* Investigated buildbot cert expiry
* [Posted message about expired cert](https://internals.rust-lang.org/t/buildbot-is-down-for-a-bit/2365)
* [Reviewed httptest patch](https://github.com/brson/httptest/pull/1)
* Worked on meld
* [Encouraged dwarf hacker](https://users.rust-lang.org/t/dwarf-debug-format/2100)

# 2015-07-14

* MC'd a production support meeting
* Sent minutes to above meeting
* [Responded to std doc feedback again](https://github.com/rust-lang/rust/pull/26977)
* [Added `#[packed]` breakage to release notes](https://github.com/rust-lang/rust/pull/27040)
* Added some more to annotated-std
* [Filed issue about linkage regression](https://github.com/rust-lang/rust/issues/27043)
* Set up new FreeBSD builders for dhuseby
* [Reviewed httptest patch](https://github.com/brson/httptest/pull/1)

# 2015-07-13

* [Reviewed errorck.py fix](https://github.com/rust-lang/rust/pull/26984#issuecomment-121011032)
* [Reviewed blastoff --yes patch](https://github.com/brson/multirust/pull/80)
* [Reviewed x86_64 abi fixes](https://github.com/rust-lang/rust/pull/27017)
* [Reviewed apple clang configure patch](https://github.com/rust-lang/rust/pull/27006)
* [Suggested a way to work around cwd issue in rustbook upgrade](https://github.com/rust-lang/rust/pull/26216#issuecomment-121019698)
* [Responded to stdx thread](https://users.rust-lang.org/t/stdx-the-missing-batteries-of-rust/2015/46)
* [Posted twir](https://www.reddit.com/r/rust/comments/3d5yta/this_week_in_rust_87/)
* [Reviewed stdx readme tweak](https://github.com/brson/stdx/pull/5)
* Responded to all pings
* [Posted regression report](https://internals.rust-lang.org/t/new-crater-reports-1-1-stable-vs-beta-2015-07-10-and-nightly-2015-07-10/2358)
* Put together agenda for prod meetings
* [Left some more comments on annotated std](https://github.com/brson/std-annotated-rs/commit/cb12c0e8758181a8f0be0671707025450eb60381)
* [Started thread on reverting `#[packed]` breakage](https://internals.rust-lang.org/t/turning-packed-into-a-permanent-warning-and-backporting-to-1-2/2359)
* [Addressed feedback on std doc patch](https://github.com/rust-lang/rust/pull/26977)
* [Described final stdx crates](https://users.rust-lang.org/t/stdx-the-missing-batteries-of-rust/2015/49?u=brson)

# 2015-07-12

* [Responded to rustle feature request](https://github.com/brson/rustle/issues/1)
* [Reviewed gedit updates](https://github.com/rust-lang/gedit-config/pull/8)
* [Reviewed stdx typo](https://github.com/brson/stdx/pull/4)
* [Responded to q about cargo bug and doing a point release](https://github.com/rust-lang/cargo/issues/1801#issuecomment-120693008)
* [Commented on rustbook pr problem](https://github.com/rust-lang/rust/pull/26216#issuecomment-120762561)
* [Reviewed tls fix](https://github.com/rust-lang/rust/pull/26958)
* [Reviewed errorck fix](https://github.com/rust-lang/rust/pull/26984)
* **twir**
* [Worked on my website](http://brson.github.io/rust-stuff.html)

# 2015-07-11

* **Writing std docs**
* [Posted PR improving std docs](https://github.com/rust-lang/rust/pull/26977)

# 2015-07-10

* Organized some accomplishments
* [Responded to q about core team re stdx](https://users.rust-lang.org/t/stdx-the-missing-batteries-of-rust/2015/35?u=brson)
* **Working on rust-installer ldconfig bug**
* [Opened issue about rust-installer default dir](https://github.com/rust-lang/rust-installer/issues/40)
* Responded to pings
* Started beta packaging rebuilds to pick up cargo ssl fix
* [Praised llogiq's writing](https://www.reddit.com/r/rust/comments/3ct5yx/blog_holy_stdborrowcow_redux/csyu60t)
* [Posted rust-installer ldconfig patch](https://github.com/rust-lang/rust-installer/pull/41)
* [Upgraded rust-installer for rust-packaging](https://github.com/rust-lang/rust-packaging/pull/38)
* [Upgraded rust-installer for cargo](https://github.com/rust-lang/cargo/pull/1798)
* [Upgraded rust-installer for rust](https://github.com/rust-lang/rust/pull/26943)
* Upgraded rust-installer for multirust
* **twir**
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/26944)
* [Reviewed rustbook merge failure](https://github.com/rust-lang/rust/pull/26216#issuecomment-120508913)

# 2015-07-09

* [Merged beta backports](https://github.com/rust-lang/rust/pull/26901)
* [Bumped beta to .2](https://github.com/rust-lang/rust/pull/26921)
* [Reviewed msvc unwind revert](https://github.com/rust-lang/rust/pull/26919)
* Started build of 1.2.0-beta.2
* [Reviewed save-analysis patch](https://github.com/rust-lang/rust/pull/26907)
* Responded to all pings
* [Responded to q about custom rust optimization pass ordering](https://internals.rust-lang.org/t/using-a-custom-optimisation-pass-pipeline/2345/5?u=brson)
* [Finished relnotes updates](https://github.com/rust-lang/rust/pull/26613)
* [Reviewed llvm ar patch](https://github.com/rust-lang/rust/pull/26926)
* [Reviewed windows directory junction fix](https://github.com/rust-lang/rust/pull/26929)
* [Posted rust-buildbot license](https://github.com/rust-lang/rust-buildbot/pull/20)
* [Posted rustup license](https://github.com/rust-lang/rustup/pull/24)
* [Posted rust-packaging license](https://github.com/rust-lang/rust-packaging/pull/36)
* [Posted rust-installer license](https://github.com/rust-lang/rust-installer/pull/39)

# 2015-07-08

* In SF, away from computer

# 2015-07-07

* [Encouraged 'easier libc in rust'](https://www.reddit.com/r/rust/comments/3cfjbp/easier_libc_in_rust/csv6g2l)
* [Closed PR to remove hyper from stdx](https://github.com/brson/stdx/pull/3#issuecomment-119277252)
* [Posted patch to revert broken stage number reporting](https://github.com/rust-lang/rust/pull/26863)
* [Commented on closure inference changes](https://github.com/rust-lang/rfcs/pull/756#issuecomment-119287742)
* [Commented on stdx thread](https://users.rust-lang.org/t/stdx-the-missing-batteries-of-rust/2015/27)
* Responded to all pings
* [Commented on Duration stabilization](https://github.com/rust-lang/rust/pull/26818#issuecomment-119295749)
* [Updated brson in rustaceans.org](https://github.com/nrc/rustaceans.org/pull/166)
* [Reviewed cargo dep install scripts](https://github.com/rust-lang/cargo/pull/1788)
* **Scheduling meetings**

# 2015-07-06

* Restarted mac3/4
* Turned in laptop for refurb again
* [Reviewed multirust ctl default patch](https://github.com/brson/multirust/pull/78#issuecomment-118940288)
* [Reviewed rustup fix](https://github.com/brson/multirust/pull/78#issuecomment-118940288)
* [Upgraded rustup in multirust](https://github.com/brson/multirust/commit/7b3ae3bf35c08f8dee6e89e692456c9e1b975c5d)
* [Started crater run for prelude feature gating](https://tools.taskcluster.net/aws-provisioner/)
* [Nominating msi problem](https://github.com/rust-lang/rust/issues/26765)
* [Nominated another msi problem](https://github.com/rust-lang/rust/issues/26758)
* [Reviewed minor formatting patch](https://github.com/rust-lang/rust/pull/26757)
* [Reviewed adding issue numbers to unstable attribute](https://github.com/rust-lang/rust/pull/26747)
* [Started twir draft](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/16)
* [Reviewed new msvc patch](https://github.com/rust-lang/rust/pull/26741)
* [Merged beta backports](https://github.com/rust-lang/rust/pull/26700)
* Started new nightly build
* [Commented on prelude_import issue](https://github.com/rust-lang/rust/issues/26690#issuecomment-118964487)
* [Commented on tarball perm issue](https://github.com/rust-lang/rust/issues/26685#issuecomment-118965857)
* [Reviewed rust-installer netbsd patch](https://github.com/rust-lang/rust-installer/pull/38)
* [Commented on signing snaps](https://github.com/rust-lang/rust/issues/13254#issuecomment-118968182)
* [Started new crater run for resolve fix](https://github.com/rust-lang/rust/pull/26242#issuecomment-118969343)
* [Told richo to add the stage number to version output only for stage0/2](https://github.com/rust-lang/rust/pull/26599)
* [Thanked @nasa42 for finishing twir](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/26?u=brson)
* [Reviewed whitespace patch](https://github.com/rust-lang/rust/pull/26834)
* [Found occurances of `&mut |` for eddyb](https://gist.github.com/brson/f8ec7bcdf54e5d18419f)
* [Rereviewed patch to add stage info to rustc version output](https://github.com/rust-lang/rust/pull/26599)
* Followed up on production user responses
* Took whistler survey
* Submitted June discourse expense
* Ordered pizza for Wednesday meetup
* [Removed flate2 and hyper from stdx](https://github.com/brson/stdx/pull/3)
* [Finished crater report for prelude feature gate](https://github.com/rust-lang/rust/pull/26699#issuecomment-119027362)
* [Finished crater report for resolve fix](https://github.com/rust-lang/rust/pull/26242#issuecomment-119027632)
* [Approved resolve fix](https://github.com/rust-lang/rust/pull/26242#issuecomment-119027651)
* Wrote more stdx docs
* [Reviewed cargo pr](https://github.com/rust-lang/cargo/pull/1776)

# 2015-07-05

* [Merged stdx feature configuration patch](https://github.com/brson/stdx/pull/2)

# 2015-07-04

* Working on stdx
* [Filed bug on pub extern crate](https://github.com/rust-lang/rust/issues/26775)
* [Publish stdx and asked for help](https://users.rust-lang.org/t/stdx-the-missing-batteries-of-rust/2015)
* [Created the Temple of Rust](http://brson.github.io/temple-of-rust/)

# 2015-07-03

* Setting up AWS security and proxy settings for crater
* Sent crater instructions out

# 2015-07-02

* More production user coordination

# 2015-07-01

* Updated deployed crater and started crater-web
* Assigned an elastic IP to crater
* Pinged more production users
* Sent out production meeting polls

# 2015-06-30

* Responded to emails
* [Thanked @nasa42 for getting twir out](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/18)
* Wrote production users meeting scheduling email
* [Nominated relnotes for beta](https://github.com/rust-lang/rust/pull/26579)
* [Nominated relnotes for beta](https://github.com/rust-lang/rust/pull/26613)

# 2015-06-29

* Writing docs for crater

# 2015-06-28

* Writing docs for crater-cli
* [Commented on multirust ctl default-toolchain patch](https://github.com/brson/multirust/pull/78)
* Sent viz folks an update about what Niko and I are thinking
* [Mentioned in twir thread that I may not be available Monday](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/15)

# 2015-06-27

* Trying to get crater-cli working
* [Reviewed rust-packaging for msvc](https://github.com/rust-lang/rust-packaging/pull/35)

# 2015-06-26

* [Told twir editors I can't work this week](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/14)
* [Reviewed PR to add stage to rustc version](https://github.com/rust-lang/rust/pull/26599#issuecomment-115945781)
* [Commented on centos ldconfig issue](https://users.rust-lang.org/t/installation-error-in-centos7/1938/5?u=brson)
* [Reviewed XP patch](https://github.com/rust-lang/rust/pull/26601)
* [Reviewed cargo bump](https://github.com/rust-lang/cargo/pull/1753)
* [Reviewed 1.2 relnote patch](https://github.com/rust-lang/rust/pull/26579)
* [Submitted my own 1.2 relnote patch](https://github.com/rust-lang/rust/pull/26613)
* [Reviewed other 32-bit msvc patch](https://github.com/rust-lang/rust/pull/26569)
* [Reviewed kde gitignore patch](https://github.com/rust-lang/rust/pull/26565)
* More work sorting out discourse payments

# 2015-06-25

* **Released 1.1**
* Responded to q about rust release notes
* [Posted crater results for resolve change](https://github.com/rust-lang/rust/pull/26242#issuecomment-115369349)
* [Posted relnotes fix](https://github.com/rust-lang/rust/pull/26574)
* Updated CC for internals.rlo

# 2015-06-24

* Discussed cross-compiling considerations
* [Reviewed error code comment](https://github.com/rust-lang/rust/pull/26545)
* Responded to somebody that wants access to calendar
* [Responded to twir thread about qotw and fott](http://www.reddit.com/r/rust/comments/3aw08g/this_week_in_rust_84/cshc4h3)
* [Posted crater report tuple struct restriction](https://github.com/rust-lang/rust/pull/26421#issuecomment-114985629)
* [Posted crater report on fat pointer restrictions](https://github.com/rust-lang/rust/pull/26394#issuecomment-114985751)
* Started crate build for #26242
* [Updated rust-packaging for 1.2](https://github.com/rust-lang/rust-packaging/pull/34)
* Started new 1.2 beta
* [Re-reviewed freebsd clang patch](https://github.com/rust-lang/rust/pull/26185)
* Shared calendar with somebody
* [Posted response to q about unused argument lint](https://github.com/rust-lang/rust/pull/26502#issuecomment-115009374)
* [Posted capitalization of std docs](https://github.com/rust-lang/rust/pull/26553)
* [Posted issue to rename rustc_unicode](https://github.com/rust-lang/rust/issues/26554)
* [Posted issue to remove to_titlecase](https://github.com/rust-lang/rust/issues/26555)
* [Nominated align_of for backport to 1.2](https://github.com/rust-lang/rust/pull/25646)
* **1.2 relnotes**

# 2015-06-23

* [Posted a negative response to documentation policy thread](https://internals.rust-lang.org/t/lets-have-a-documentation-policy/2279/2?u=brson)
* [Responded to multirust source install issue](https://github.com/brson/multirust/issues/77#issuecomment-114592731)
* [Reviewed PATH frobbing PR](https://github.com/rust-lang/rust/pull/26490)
* [Reviewed fix for host triples as subset of build triples](https://github.com/rust-lang/rust/pull/26491)
* [r- removing unused arguments from default methods](https://github.com/rust-lang/rust/pull/26502#issuecomment-114594595)
* [Commented on OS X installer size reporting](https://github.com/rust-lang/rust/issues/26501#issuecomment-114595005)
* [Commented on new PNaCL PR](https://github.com/rust-lang/rust/pull/26505)
* Responded to all ping
* Building crates for two outstanding crater runs
* [Reviewed PR to add twitter account to twir](https://github.com/cmr/this-week-in-rust/pull/70)
* [Starting crater run for resolve changes](https://github.com/rust-lang/rust/pull/26242)
* [Uploaded 1.1 to staging](https://static.rust-lang.org/dist/staging/index.html)
* [Posted 1.1 testing request](https://internals.rust-lang.org/t/1-1-rc-testing/2284)
* [Reviewed deprecation inheritance](https://github.com/rust-lang/rust/pull/26061)
* [Bumped master to 1.3](https://github.com/rust-lang/rust/pull/26527)
* Started 1.2 beta build
* [Posted update to research papers](https://github.com/rust-lang/rust/pull/26528)
* [Published twir](http://this-week-in-rust.org/blog/2015/06/22/this-week-in-rust-84/)
* [Reviewed cargo msvc patch](https://github.com/rust-lang/cargo/pull/1724)
* [Replied again to multirust source request](https://github.com/brson/multirust/issues/77#issuecomment-114691114)

# 2015-06-21

* **Working on crater-cli communicating with crater-web**

# 2015-06-20

* [Merged twir updates](https://github.com/cmr/this-week-in-rust/pull/68)
* [Merged twir typo](https://github.com/cmr/this-week-in-rust/pull/65)
* [Merged twir updates](https://github.com/cmr/this-week-in-rust/pull/67)
* [Merged twir updates](https://github.com/cmr/this-week-in-rust/pull/69)
* [Posted PR to put std link higher in the doc index](https://github.com/rust-lang/rust/pull/26462)
* [Updated archaea](https://users.rust-lang.org/t/watching-rust-evolve-changelog/1877/4?u=brson)
* **Working on command line parsing for crater-cli**

# 2015-06-19

* Reviewed Niko's new stability RFC
* [Posted 1.1 backports](https://github.com/rust-lang/rust/pull/26436)
* [Posted merge into stable](https://github.com/rust-lang/rust/pull/26437)
* [Reviewed rust-www s3 deployment fixes](https://github.com/rust-lang/rust-www/pull/150)
* Moved macs to new location for the ww because of electrical work
* [Started crater run for tuple struct resolution fix](https://github.com/rust-lang/rust/pull/26421)
* [Started crater run for fat pointer casts](https://tools.taskcluster.net/task-inspector/#jpj1s_UlTX6RWTMz4ys3Uw)
* Responded to all pings
* [Responded to q's about debugging homu](https://github.com/barosl/homu/pull/67#issuecomment-82275752)
* [Merged doc backport](https://github.com/rust-lang/rust/pull/26440)

# 2015-06-18

* [Posted a quote for twir](https://users.rust-lang.org/t/twir-quote-of-the-week/328/85?u=brson)
* [Commented about wasm + Rust](http://www.reddit.com/r/rust/comments/3abgbo/webassembly_rust_compile_to_web/csb2z6h)
* [Made minor updates to relnotes](https://github.com/rust-lang/rust/pull/26200)
* **Preparing backports to do last 1.1 beta**
* [Posted comment to not backport a dst fix](https://github.com/rust-lang/rust/pull/26038)
* [Posted 1.1 backports](https://github.com/rust-lang/rust/pull/26409)
* Started build of 1.1.0-beta.4
* [Reviewed edunham's www https plan](https://github.com/rust-lang/rust-www/issues/148)
* [Reviewed a configure fix](https://github.com/rust-lang/rust/pull/26381)
* [Reviewed rustc PATH fixes](https://github.com/rust-lang/rust/pull/26382)
* [Made suggestions about the pnacl patch](https://github.com/rust-lang/rust/pull/26148)
* [Reviewed extended error patch](https://github.com/rust-lang/rust/pull/26393)
* [Reviewed extended error patch](https://github.com/rust-lang/rust/pull/26399)
* [Reviewed patch to disable landing pads on msvc](https://github.com/rust-lang/rust/pull/26414)
* Responded to all pings
* [Posted patch to fix stable feature gate error](https://github.com/rust-lang/rust/pull/26417)
* Updated workday q2 goals
* [Started a twir draft](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/13?u=brson)
* [Posted encouragement to maidsafe thread](https://forum.safenetwork.io/t/maidsafe-dev-update-8th-june-2015/4069/31?u=brson)
* [Responded to pre-rfc about publishing expanded syntax extensions](https://internals.rust-lang.org/t/pre-rfc-stablized-syntax-extensions-sort-of/2259/4?u=brson)
* [Replied to thread about 'notable' tags](https://internals.rust-lang.org/t/adding-a-notable-tag-to-rust-lang-rust-for-prs/2234/7)
* [Replied to thread about compiler rfc policy](https://internals.rust-lang.org/t/request-for-feedback-rfc-policy/2255/2)
* [Replied to thread about to_upper breakage](https://internals.rust-lang.org/t/stable-breaking-changes-to-to-upper-to-lower/2247/3?u=brson)
* [Started list of 1.2 breaking changes](https://gist.github.com/brson/988abf1d1db005b50d71)
* [Responded to question of to_upper breakage](https://github.com/rust-lang/rust/pull/26039#issuecomment-113336098)
* [Closed old homu PR](https://github.com/barosl/homu/pull/67)

# 2015-06-17

* [Added Hanover meetup to calendar](http://www.reddit.com/r/rust/comments/3a63tt/rust_meetup_in_hanover_germany/)
* [Praised the error generating macro](http://www.reddit.com/r/rust/comments/3a655c/darkfox_shares_with_us_a_macro_to_automagically/cs9ta5i)
* [Reviewed beta backport](https://github.com/rust-lang/rust/pull/26371)
* [Fixing UI of unstable_feature lint](https://github.com/rust-lang/rust/pull/26371)
* Responded to remo council about rust meetup funding. Doesn't look good.
* Responded to somebody about rust visualizations
* Finished beta regression report
* Rebased 1.1 release notes patch

# 2015-06-16

* Responded to morning pings
* [Responded to suggestion in twir thread](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806/5?u=brson)
* Sent emails to production users inviting them to talk
* Resolved a security@ email
* **TWiR**
* [Reviewed twir redesign](https://github.com/cmr/this-week-in-rust/pull/66)
* [Reviewed stable cargo pr](https://github.com/rust-lang/cargo/pull/1712)
* [Posted encouraging comment about wayland-client](http://www.reddit.com/r/rust/comments/3a2e0u/the_crate_waylandclient_now_020_and_in_a_fairly/)
* [Posted TWiR](http://www.reddit.com/r/rust/comments/3a3ftu/this_week_in_rust_83/)

# 2015-06-15

* [Reviewed cargo retry patch](https://github.com/rust-lang/cargo/pull/1711#issuecomment-112153975)
* [Started the twir editor's thread](https://users.rust-lang.org/t/this-week-in-rust-editors-thread/1806)
* Responded to morning pings
* Updated easydns account secrets
* Gave acrichto access to brson/cargo
* Talking to reps council about meetups

# 2015-07-14

* **Continuing to polish linkage patch**
* [Reviewed regex license patch](https://github.com/rust-lang/regex/pull/90)
* [Posted a regression report](https://internals.rust-lang.org/t/regression-report-stable-2015-05-15-vs-nightly-2015-06-12/2240)

# 2015-06-13

* **Cleaning up monomorphization linkage patch**
* **Running more crater tests**

# 2015-06-12

* [Reviewed cargo fix](https://github.com/rust-lang/cargo/pull/1701#issuecomment-111555125)
* [Responded to thread about 'notable' tag](https://internals.rust-lang.org/t/adding-a-notable-tag-to-rust-lang-rust-for-prs/2234/6?u=brson)
* [Bumped beta version](https://github.com/rust-lang/rust/pull/26254)
* Started a beta
* [Reviewed a resolve patch](https://github.com/rust-lang/rust/pull/26242)
* [Added a relnotes tag](https://internals.rust-lang.org/t/adding-a-notable-tag-to-rust-lang-rust-for-prs/2234/5)
* [Linked some resolve issues](https://github.com/rust-lang/rust/issues/4865#issuecomment-111599072)
* [Commented on backtrace path patch](https://github.com/rust-lang/rust/pull/26252#issuecomment-111599958)
* [Reviewed LLVM upgrade](https://github.com/rust-lang/rust/pull/26025)
* [Reviewed test cleanup](https://github.com/rust-lang/rust/pull/26253)
* Talked to Vikrant about TWiR maintenance
* More collection of production contacts
* Scheduled meeting to talk to Joseph Cotton about visualization research
* [Reviewed playpen docs](https://github.com/rust-lang/rust-playpen/pull/132)
* Started another internal fundraising thread
* Started crater test stable-2015-05-15 vs. nightly-2015-06-12
* [Closed old valgrind pr](https://github.com/rust-lang/rust/pull/24477)
* [Reviewed rust-installer bitrig fix](https://github.com/rust-lang/rust-installer/pull/37)
* [Reviewed cargo patch](https://github.com/rust-lang/cargo/pull/1709)
* [Posted some benchmarks](http://www.reddit.com/r/rust/comments/39mqen/rust_vs_benchmarks/cs4r680)
* Writing intro message to production users

# 2015-06-11

* [Commented on compile time improvement](http://www.reddit.com/r/rust/comments/39f21l/out_of_curiosity_who_is_currently_focusing_on/cs3caw7)
* Put together list of production users
* Responded to contributor about sponsoring twir
* Sent message to rust-community about expanding twir
* [Posted thread about 'notable' tag](https://internals.rust-lang.org/t/adding-a-notable-tag-to-rust-lang-rust-for-prs/2234)
* [Reviewed cargo fix](https://github.com/rust-lang/cargo/pull/1705#event-328197890)
* [Reviewed MSVC packaging fix](https://github.com/rust-lang/rust/pull/26226)

# 2015-06-10

* [Published new crater results for Extend PR](https://github.com/rust-lang/rust/pull/26122#issuecomment-110843483)
* [Reviewed cargo patch](https://github.com/rust-lang/cargo/pull/1702)
* [Commented on fixes to linguist for Rust](https://github.com/github/linguist/pull/2447)
* [Commented on rustup concurrency issues](https://github.com/brson/multirust/issues/76)
* [Reviewed std::process cleanup](https://github.com/rust-lang/rust/pull/26159)
* Upgraded rustup.sh in multirust
* Responded to question about offline use of rustup.sh
* Talking to richard about airmo registration problems
* Rebooted mac4
* [Left a comment about deprecation inheritance](https://github.com/rust-lang/rust/pull/26061#issuecomment-110093925)
* [Reviewed freebsd clang patch](https://github.com/rust-lang/rust/pull/26185#issuecomment-110874318)
* [Responded to tools triage](https://internals.rust-lang.org/t/tools-triage-for-2015-06-09/2226)
* Set minimum post length to 1 on users.rlo and internals.rlo
* [Reviewed clang fallback patch](https://github.com/rust-lang/rust/pull/26185#issuecomment-110900721)
* Scheduled meeting with jonas to ask some questions about taskcluster
* [Responded to q about broken crates](http://www.reddit.com/r/rust/comments/39658b/trying_to_use_a_stable_release/cs27oe6)
* [Commented on type macros rfc](https://github.com/rust-lang/rfcs/pull/873#issuecomment-110915724)
* [Commented on prelude rfc](https://github.com/rust-lang/rfcs/pull/890#issuecomment-110916072)
* [Merged zsh relicense](https://github.com/rust-lang/zsh-config/pull/1)
* [Merged nano relicense](https://github.com/rust-lang/nano-config/pull/2)
* [Responded to q about building rustbook](https://internals.rust-lang.org/t/build-rustbook-without-having-to-wait-20-min/2228/2?u=brson)
* Sent steveklabnik the @rustlang twitter key
* Shipped stickers to Brazil
* [Pinged some people to help a newbie](https://github.com/rust-lang/rust/issues/25574#issuecomment-110953770)
* [Reviewed FreeBSD fix](https://github.com/rust-lang/rust/pull/26197)
* [Commented on PNaCl patch](https://github.com/rust-lang/rust/pull/26148#issuecomment-110955776)
* Sent Gerv a question about licensing markdown parser as only MIT
* [Downgraded cargo](https://github.com/rust-lang/rust-packaging/pull/33)
* [Updated 1.1 relnotes](https://github.com/rust-lang/rust/pull/26200)

# 2015-06-09

* [Pushed twir branch](http://www.reddit.com/r/rust/comments/393u6x/this_week_in_rust_82/cs0m15o)
* Pushed fixes to twir
* [Merged twir fix and republished](https://github.com/cmr/this-week-in-rust/pull/64)
* [Updated cargo for 1.1 beta](https://github.com/rust-lang/rust-packaging/pull/32)
* [Posted comment about stable compat](http://www.reddit.com/r/rust/comments/39658b/trying_to_use_a_stable_release/cs0rtd8)
* [Reviewed MSVC /MD patch](https://github.com/rust-lang/rust/pull/25995)
* Doing crater run for niko's simplified-variance-and-projection-lifetimes patch
* Doing crater for https://tools.taskcluster.net/task-inspector/#6sWS08fMRv2D422IsVSESw
* [Reviewed minor doc fix](https://github.com/rust-lang/rust/pull/26129)
* [Reviewed save-analysis fixes](https://github.com/rust-lang/rust/pull/26110)
* [Replied about rust stickers](http://www.reddit.com/r/rust/comments/3963eu/rust_in_brazil_the_ror_project/cs10xjl)
* [Merged CARGO_HOME patch to multirust](https://github.com/brson/multirust/pull/74)
* [Reviewed whitespace fix](https://github.com/rust-lang/rust/pull/26111)
* Responded to yichoi
* Scheduled room, airmo, and workplace resources for 7/8 sf meetup
* [Reviewed doc patch](https://github.com/rust-lang/rust/pull/26145)
* [Commented on missing unused features warning](https://github.com/rust-lang/rust/issues/26135)
* [Nominated implementation of Extend for beta](https://github.com/rust-lang/rust/pull/25989#issuecomment-110475529)
* [Posted binaries for object bounds crater run](https://internals.rust-lang.org/t/pre-rfc-adjust-default-object-bounds/2199/35?u=brson)
* [Reviewed char inlining](https://github.com/rust-lang/rust/pull/26154)
* [Responded to multirust environment feature request](https://github.com/brson/multirust/issues/75#issuecomment-110526284)
* [Finished crater run for niko](https://gist.github.com/brson/7d8dc603b9f676743374)
* [Posted crater run for some PR](https://github.com/rust-lang/rust/pull/26122#issuecomment-110528173)
* [Filed issue to re-sandbox playpen](https://github.com/rust-lang/rust-playpen/issues/134)

# 2015-06-08

* Posted status update
* Wrote bjz a recommendation
* [Reviewed deprecation inheritance](https://github.com/rust-lang/rust/pull/26061)
* [Posted new comments on deprecation inheritance](https://github.com/rust-lang/rust/pull/26061#issuecomment-110088790)
* Responded to question about spam filter on users.rlo
* [Filed PR to add license to gedit-config](https://github.com/rust-lang/gedit-config/pull/5)
* [Commented about licensing issues around gedit-config](https://github.com/rust-lang/gedit-config/issues/4#issuecomment-110092841)
* Allowed aws admins to see billing
* Updated q2 goals in workday again
* [Posted beta patch to fix beta naming again](https://github.com/rust-lang/rust/pull/26106)
* [Reviewed doc patch](https://github.com/rust-lang/rust/pull/26108)
* [Triaged doc pr](https://github.com/rust-lang/rust/pull/26113)
* [Reviewed fix for cargo rebuilds](https://github.com/rust-lang/cargo/pull/1697)
* Shipped stickers to montreal
* [Reviewed msvc dist patch to buildbot](https://github.com/rust-lang/rust-buildbot/pull/18)
* [Commented on likely/unlikely intrinsics](https://github.com/rust-lang/rfcs/pull/1131#issuecomment-110173092)
* [Posted twir](http://www.reddit.com/r/rust/comments/393u6x/this_week_in_rust_82/)

# 2015-06-07

* [Ignored a test blocking nightlies](https://github.com/rust-lang/rust/pull/26070)
* [Responded to q about tool subteam category](https://internals.rust-lang.org/t/internal-discussion-for-the-tools-infrastructure-team/2212/6)
* [Commented about bindgen](https://internals.rust-lang.org/t/tools-infrastructure-priorities/2213/4?u=brson)
* [Posted patch to stop compressing metadata](https://github.com/rust-lang/rust/pull/26081)
* Restarted porting crater to Rust

# 2015-06-06

* **Cleaning up monomorphization linkage patch**
* [Posted regression report for object lifetime bounds](https://internals.rust-lang.org/t/pre-rfc-adjust-default-object-bounds/2199/24?u=brson)
* **Testing patch to remove redundant metadata compression**

# 2015-06-05

* [Closed 1.0 video bug](https://bugzilla.mozilla.org/show_bug.cgi?id=1146672)
* [Reviewed LLVM upgrade](https://github.com/rust-lang/rust/pull/26025)
* [Responded to RFC to break default object bounds](https://internals.rust-lang.org/t/pre-rfc-adjust-default-object-bounds/2199/20)
* Started crater run to test better-object-bounds
* [Triaged issue](https://github.com/rust-lang/rust/issues/24622)
* Sent 100 stickers to Manish
* Bumped beta version to beta.2
* **Still trying to optimize metadata on odr patch**
* **Testing removing inline(always) on servo**
* [Tried to help with an android failure](https://github.com/rust-lang/rust/pull/25784#issuecomment-109447363)
* [Posted tests of converting inline(always) in servo](https://gist.github.com/brson/b48dd03b06c406be68e6)
* [Posted patch removing inline(always) from servo](https://github.com/servo/servo/pull/6297)
* [Posted initial comment about crate subtree patch](https://github.com/rust-lang/rust/pull/26042#issuecomment-109460818)
* Started a beta build

# 2015-06-04

* [Replied to multirust issue on spaces in paths](https://github.com/brson/multirust/issues/71#issuecomment-108982981)
* [Reviewed cargo custom path patch](https://github.com/rust-lang/cargo/pull/1657#issuecomment-108749205)
* [Filed multirust issue about --link-local](https://github.com/brson/multirust/issues/72)
* [Filed multirust issue about 'run'](https://github.com/brson/multirust/issues/73)
* [Responded to q about meetup fees](https://users.rust-lang.org/t/welcome-to-the-rust-programming-language-forum/1688/2)
* [Reviewed msvc fix](https://github.com/rust-lang/rust/pull/26004)
* [Commented about unicode conversion iterators](https://internals.rust-lang.org/t/pre-rfc-stabilize-utf-16-encoding-in-std/2152/5?u=brson)
* [Asked raphlinus to add MIT license to his markdown parser](https://users.rust-lang.org/t/new-commonmark-parser/1690/13?u=brson)
* [Commented about the markdown license again](http://www.reddit.com/r/rust/comments/38hft5/new_commonmark_parser/crvklhd)
* [Responded to submodule thread](https://internals.rust-lang.org/t/submodules-in-rust-lang-rust-for-external-repositories/2200/3)
* [Reviewed adjustment to box help message](https://github.com/rust-lang/rust/pull/26014#issuecomment-109047358)
* **Still optimizing linkage of monomorphic functions**
* Doing a crater run for niko

# 2015-06-03

* [Added periodic table and iter cheat sheet to rust-learning](https://github.com/ctjhoa/rust-learning/pull/14)
* [Praised the iter cheat sheet](http://www.reddit.com/r/rust/comments/38dl0o/rust_iterator_cheat_sheet/cru7vsj)
* [Merged fix to my web site](https://github.com/brson/brson.github.com/pull/2)
* [Nominated --version change for beta](https://github.com/rust-lang/rust/pull/25821)
* [Closed sh -> bash PR](https://github.com/rust-lang/rust/pull/25889)
* [Published new rust-sdl](https://github.com/brson/rust-sdl/issues/166)
* Finished a crater run for niko
* [Merged a playpen PR to add intel syntax](https://github.com/rust-lang/rust-playpen/pull/127)
* [Merged a playpen fix](https://github.com/rust-lang/rust-playpen/pull/128)
* Added edunham to rust-push
* [Responded to q about download stats](https://github.com/rust-lang/rust/issues/25978#issuecomment-108663511)
* **Profiling my monomorphization patch. Still a perf regression**

# 2015-06-02

* [Responded to questions about multirust's use of gpg](https://github.com/Homebrew/homebrew/pull/39224#issuecomment-108042013)
* [Reviewed patch to haskell platform installer](https://github.com/haskell/haskell-platform/pull/137)
* [Commented on haskell platform bug](https://github.com/rust-lang/rust-installer/issues/36#issuecomment-108044353)
* [Filed another bug against haskell platform installer](https://github.com/haskell/haskell-platform/issues/178)
* [Reviewed mk fix](https://www.github.com/rust-lang/rust/pull/25970)
* **Working on odr linkage opts**
* [Submitted rust-root servo patch](https://github.com/servo/servo/pull/6267)

# 2015-06-01

* Posted status report
* Tried to resolve Indian shipping issues
* [Merged multirust typo](https://github.com/brson/multirust/pull/69)
* [Commented on multirust dylink bug](https://github.com/brson/multirust/issues/43#issuecomment-106758695)
* [Merged multirust doc fix](https://github.com/brson/multirust/pull/68)
* [Reviewed gdb pretty-printer cleanup](https://github.com/rust-lang/rust/pull/25905)
* [Commented on converting configure to bash](https://github.com/rust-lang/rust/pull/25889)
* [Reviewed windows path fix](https://github.com/rust-lang/rust/pull/25868)
* [Reviewed disable-tls option](https://github.com/rust-lang/rust/pull/25858)
* [Reviewed removal of build date from --version](https://github.com/rust-lang/rust/pull/25821)
* [Commented about align_of change](https://github.com/rust-lang/rust/pull/25646#issuecomment-107669675)
* [Reviewed community translations](https://github.com/rust-lang/rust/pull/25751)
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/25947)
* [Filed weird interaction with rust-installer and haskell platform](https://github.com/rust-lang/rust-installer/issues/36)
* [Reviewed msvc bootstrap patch](https://github.com/rust-lang/rust/pull/25848)
* **Working on optimizations**

# 2015-05-31

* [Responded to thread about testing code that isn't on crates.io](https://internals.rust-lang.org/t/regression-report-stable-2015-05-15-vs-nightly-2015-05-28/2157/8)

# 2015-05-28

* Working on crater report
* [Created an I-stable-regression tag](https://internals.rust-lang.org/t/added-an-i-stable-regression-tag/2153/1)
* [Tagged a regression](https://github.com/rust-lang/rust/issues/25310)
* [Published regression report](https://internals.rust-lang.org/t/regression-report-stable-2015-05-15-vs-nightly-2015-05-28/2157)

# 2015-05-27

* Had meeting with edunham
* [Merged contributing guide for rust-buildbot](https://github.com/rust-lang/rust-buildbot/pull/17)

# 2015-05-26

* [Merged blastoff fixes](https://github.com/brson/multirust/pull/67)
* [Responded to cargo complaints thread](https://internals.rust-lang.org/t/how-about-changing-lib-to-lib-to-allow-multiple-library-in-a-crate/2022/13)
* [Encouraged a Brazillian organizer](https://users.rust-lang.org/t/projects-to-spread-rust-in-brazil/1575/2)
* [Merged multirust doc patch](https://github.com/brson/multirust/pull/66)
* [Merged rust-infra pr](https://github.com/brson/rust-infra/pull/1)
* [Filed bug about removing build date from --version](https://github.com/rust-lang/rust/issues/25812)
* [Filed bug to add recvert to signing key](https://github.com/rust-lang/rust/issues/25814)
* [Filed bug to use keybase.io](https://github.com/rust-lang/rust/issues/25815)
* [Commented on 404 issue](https://github.com/rust-lang/rust/issues/25794#issuecomment-105727169)
* [Commented on windows msys path issues](https://github.com/rust-lang/rust/issues/25781#issuecomment-105727813)
* [Commented about panic handling](https://github.com/rust-lang/rfcs/pull/1100#issuecomment-105735540)
* [Closed issue about broken install](https://github.com/rust-lang/rust/issues/25699)
* [Nominated issue of rustc accepting [[T]] syntax](https://github.com/rust-lang/rust/issues/25692)
* [Commented about man page outdatedness](https://github.com/rust-lang/rust/issues/25689#issuecomment-105738575)
* [Commented on --test-threads](https://github.com/rust-lang/rust/issues/25636#issuecomment-105739287)
* [Commented on cfg_attr(path)](https://github.com/rust-lang/rust/issues/25544#issuecomment-105740210)
* [Commented on source tarball generation](https://github.com/rust-lang/rust/issues/25479)
* [Commented on rust-buildbot spot instance PR](https://github.com/rust-lang/rust-buildbot/pull/16#issuecomment-105743274)

# 2015-05-25

* [Reviewed blog post update](https://github.com/rust-lang/blog.rust-lang.org/pull/60)
* [Reviewed multirust dirname fix](https://github.com/brson/multirust/pull/65)
* [Commented on multirust override issue](https://github.com/brson/multirust/issues/54#issuecomment-104880613)
* [Merged multirust doc patch](https://github.com/brson/multirust/pull/64)
* Updated rust-admin repo with a README indicating it's mostly deprecated
* **Creating infrastructure documentation for edunham**
* Sent edunham invites to rust mtg and triage
* [Nominated eddyb's issue](https://github.com/rust-lang/rust/issues/25776)
* [Reviewed rust-www fixes](https://github.com/rust-lang/rust-www/pull/142)
* [Reviewed rust-www https links](https://github.com/rust-lang/rust-www/pull/143)
* [Reviewed math pr](https://github.com/rust-lang/rust/pull/25780)
* Created aws account for edunham
* [Added detail to feature upload issue](https://github.com/rust-lang/rust/issues/25724)
* [Added detail about manifest metadata](https://github.com/rust-lang/rust/issues/21243#issuecomment-105298244)
* [Closed dist indexing issue](https://github.com/rust-lang/rust/issues/21241)
* Sent edunham a braindump

# 2015-05-22

* [Posted useful link to datetime thread](https://github.com/rust-lang/rfcs/issues/619)
* [Reviewed cargo PR](https://github.com/rust-lang/cargo/pull/1617#event-307968643)
* [Retried cargo PR](https://github.com/rust-lang/cargo/pull/1641)
* [Pinged aturon on align_of breaking change](https://github.com/rust-lang/rust/pull/25646#issuecomment-104711760)
* [Responded to thread about mobile strategy](https://internals.rust-lang.org/t/what-about-a-strategy-for-mobile-platforms/2115/3?u=brson)
* Unstuck homu
* [Reviewed config patch](https://github.com/rust-lang/rust/pull/25697)
* [Posted fix for make install](https://github.com/rust-lang/rust/pull/25717)
* Set up freebsd slave configs for koobs
* [Fixing nightly cross-compile bug](https://github.com/rust-lang/rust/pull/25719)
* [Reviewed doc patch](https://github.com/rust-lang/rust/pull/25695)
* [Reviewed clang 3.7 patch](https://github.com/rust-lang/rust/pull/25722)
* [Filed bug about cross breakage](https://github.com/rust-lang/rust/issues/25723)

# 2015-05-21

* [Reviewed align_of patch](https://github.com/rust-lang/rust/pull/25646)
* [Reviewed dirent fix](https://github.com/rust-lang/rust/pull/25632)
* [Triaged doc pr](https://github.com/rust-lang/rust/pull/25666)
* [Merged and deployed rust-buildbot cargo bitrig patch](https://github.com/rust-lang/rust-buildbot/pull/14)
* [Merged playpen patch](https://github.com/rust-lang/rust-playpen/pull/111)
* [Closed competing patch](https://github.com/rust-lang/rust-playpen/pull/107)
* Redeployed play.rlo
* [Merged another play fix](https://github.com/rust-lang/rust-playpen/pull/114)
* [Reviewed multirust doc patch](https://github.com/brson/multirust/pull/64)
* [Merged cargo patch](https://github.com/rust-lang/cargo/pull/1640#event-309204885)
* [Reviewed cargo patch](https://github.com/rust-lang/cargo/pull/1641#event-309194038)

# 2015-05-20

* [Reviewed minor playpen style fixes](https://github.com/rust-lang/rust-playpen/pull/110)
* [Reviewed playpen #[test] detection](https://github.com/rust-lang/rust-playpen/pull/111)
* [Reviewed playpend #[test] support](https://github.com/rust-lang/rust-playpen/pull/107)
* Submitted PTO
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/57)
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/58)
* Redeployed twir
* Updated workday deliverables
* [Reviewed doc patch](https://github.com/rust-lang/rust/pull/25656)
* [Submitted kate licensing patch](https://github.com/rust-lang/kate-config/pull/9)

# 2015-05-19

* [Thanked wting for sharing party photos](http://www.reddit.com/r/rust/comments/36gagk/rust_10_launch_party_photos/)
* Restarted mac4
* **Working on twir**
* [Fixed beta versioning](https://github.com/rust-lang/rust/pull/25620)
* [Reviewed rust-buildbot msvc patch](https://github.com/rust-lang/rust-buildbot/pull/13)
* [Posted meeting minutes](https://internals.rust-lang.org/t/weekly-meetings-2015-05-19-fott-servo-diversity-subteams/2085)
* [Responded to rust-buildbot pr](https://github.com/rust-lang/rust-buildbot/pull/14)
* [Reviewed ar patch](https://github.com/rust-lang/rust/pull/25411)
* [Posted twir](http://www.reddit.com/r/rust/comments/36jwbp/this_week_in_rust_81/)

# 2015-05-18

* Fixed beta naming in rust-packaging
* [Submitted PR to rust-learning updating macros link](https://github.com/ctjhoa/rust-learning/pull/9)
* [Bumped cargo version](https://github.com/rust-lang/cargo/pull/1627)
* Shipped t-shirts to Paige and kmc
* [Commented on multirust doc feature request](https://github.com/brson/multirust/issues/55#issuecomment-103044816)
* [Reviewed play.rlo gist PR](https://github.com/rust-lang/rust-playpen/pull/102)
* [Closed rust-www beta link issue](https://github.com/rust-lang/rust-www/issues/136)
* [Finished reviewing msvc patch](https://github.com/rust-lang/rust/pull/25350)
* [Commented about deficiencies in cargo](https://internals.rust-lang.org/t/how-about-changing-lib-to-lib-to-allow-multiple-library-in-a-crate/2022/6?u=brson)
* [Reviewed multirust typo](https://github.com/brson/multirust/pull/63)
* [Reviewed cargo RUSTC PR](https://github.com/rust-lang/cargo/pull/1629)
* [Reviewed macro backtrace PR](https://github.com/rust-lang/rust/pull/25520#issuecomment-103285169)
* [Reviewed cargo dotfile PR](https://github.com/rust-lang/cargo/pull/1625)
* [Merged playpen gist pr](https://github.com/rust-lang/rust-playpen/pull/102)
* Deployed playpen
* [Merged another playper pr](https://github.com/rust-lang/rust-playpen/pull/106)
* Deployed playpen

# 2015-05-17

* Mostly worked on linkonce odr stuff

# 2015-05-16

* Fixing build problem on beta
* [Filed PR fixing tests on beta](https://github.com/rust-lang/rust/pull/25504)
* [Filed same PR on master](https://github.com/rust-lang/rust/pull/25503)
* Started another beta build
* [Gathered stats about monomorphization in servo](https://gist.github.com/brson/ce72710c48ef060dba59)
* [Reviewed playpen fix](https://github.com/rust-lang/rust-playpen/pull/100#issuecomment-102649654)

# 2015-05-15

* [Bumped version to 1.2](https://github.com/rust-lang/rust/pull/25447)
* Started build for 1.1.0-beta.1
* [Merged minor www tweak](https://github.com/rust-lang/rust-www/pull/129)
* [Merged fix to my-first-contribution](https://github.com/brson/my-first-contribution/pull/1)

# 2015-05-14

* [Filed packaging bug on windows signing](https://github.com/rust-lang/rust-packaging/issues/31)
* Uploaded 1.0 artifacts for testing
* Updated security key with revocation cert
* Uploaded 1.0 docs to stable/
* Adjusted nginx to support stable docs
* Sent out new security team keys
* [Uploaded security key to static.rlo](http://static.rust-lang.org/rust-security.gpg.asc)
* [Posted PR to rust-learning adding Error Handling in Rust](https://github.com/ctjhoa/rust-learning/pull/7)
* [Posted another PR to rust-learning](https://github.com/ctjhoa/rust-learning/pull/8)
* Tested the stable build of Rust
* [Updated 1.0 www PR](https://github.com/rust-lang/rust-www/pull/120)
* Sent Jess guestlist
* Writing an app to display contributors' first contributions
* [Updated rustup.sh for stable](https://github.com/rust-lang/rustup/pull/16)
* [Filed trpl stack frame illustration bug](https://github.com/rust-lang/rust/issues/25428)

# 2015-05-13

* Restarted mac4
* Started 1.0 stable build
* [Commented on Result::expect](https://github.com/rust-lang/rust/pull/25359#issuecomment-101806317)
* [Reviewed ar fix](https://github.com/rust-lang/rust/pull/25238)
* [Reviewed filename hash fix](https://github.com/rust-lang/rust/pull/25208)
* [Closed --no-capture PR](https://github.com/rust-lang/rust/pull/24451)
* [Assigned a bug to acrichto](https://github.com/rust-lang/rust/pull/24741)
* [Reopened --no-capture PR](https://github.com/rust-lang/rust/pull/24451#issuecomment-101808210)
* [Replied to thread about detecting llvm version](https://users.rust-lang.org/t/determine-what-version-of-llvm-compiled-rustc/1345)
* Created security@rust-lang.org
* [Created security gpg key](https://pgp.mit.edu/pks/lookup?op=vindex&search=0xEFB9860AE7520DAC)
* Ordered pizza for meetup

# 2015-05-12

* Continuing crater build for niko
* Testing stable release in dev
* [Filed multirust run issue](https://github.com/brson/multirust/issues/62)
* [Updated std doc PR](https://github.com/rust-lang/rust/pull/25224)
* [Reviewed AUTHORS.txt update](https://github.com/rust-lang/rust/pull/25253)
* Cherry-picked AUTHORS.txt update to beta
* [Reviewed archive handling fix](https://github.com/rust-lang/rust/pull/25238)
* [Reviewed CFG_FILENAME_EXTRA patch](https://github.com/rust-lang/rust/pull/25208)
* [Reviewed book release channels patch](https://github.com/rust-lang/rust/pull/25174)
* Fixed some bugs in crater
* Sent t-shirt shipment info to final organizers
* Responded to Mike Poessey about meetup A/V requirements
* [Posted link about packt book](http://www.reddit.com/r/rust/comments/35qqzv/rust_essentials_packt_books/)
* Responded to Ivo the author of the packt book
* Two crater runs suffered DNS failures. Doing another.
* [Responded to mutex poisoning thread](https://internals.rust-lang.org/t/mutex-locking-and-poisoning/2019)
* [Posted minutes](https://internals.rust-lang.org/t/weekly-meetings-2015-05-12-servo-crates-io-meetings-and-subteams/2020/1)
* Finished niko's crater run
* [Posted cargo retry feature request](https://github.com/rust-lang/cargo/issues/1602)

# 2015-05-11

* [Added Montreal event to list](https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171)
* [Did more adjustments to AUTHORS.txt](https://github.com/rust-lang/rust/pull/25196)
* Talked up Rust 1.0 at the Monday meeting
* [Merged a backport PR](https://github.com/rust-lang/rust/pull/25299)
* [Merged another backport](https://github.com/rust-lang/rust/pull/25311)
* [Bumped prerelease version for another beta](https://github.com/rust-lang/rust/pull/25312)
* Fixed breakage on beta
* [Responded to cargo-lock-to-dot issue](https://github.com/brson/cargo-lock-to-dot/issues/2)
* [Merged cargo-lock-to-dot issue](https://github.com/brson/cargo-lock-to-dot/pull/1)
* [Reviewed multirust fix](https://github.com/brson/multirust/pull/61)
* Responded to Koobs about freebsd buildslaves
* [Posted servo cleanup PR](https://github.com/servo/servo/pull/6010)
* Sent tracking numbers to organizers
* Talked to mitchell about a few rust pr ideas
* Responded to mw about various things
* Requested status update on cake
* Started a crater build for niko

# 2015-05-10

* [Updated AUTHORS PR](https://github.com/rust-lang/rust/pull/25196)
* [Posted beta relnote pr](https://github.com/rust-lang/rust/pull/25294)

# 2015-05-08

* Alerted bvssvni about https://github.com/rust-lang/rust/pull/25157
* [Removed beta-nominated tags from merged backports](https://github.com/rust-lang/rust/pull/25192)
* [Reviewed mem::forget](https://github.com/rust-lang/rust/pull/25187)
* **[Posted www updates](https://github.com/rust-lang/rust-www/pull/120)**
* [Reviewed distcheck fix](https://github.com/rust-lang/rust/pull/25217)
* [Responded to thread about fedora install](https://www.reddit.com/r/rust/comments/35axcd/unable_to_install_rust_on_fedora_21/cr2siq4)
* [Filed rust-installer NixOS issue](https://github.com/rust-lang/rust-installer/issues/33)
* [Sent PR to rust-learning to credit japaric](https://github.com/ctjhoa/rust-learning/pull/2)
* [Reviewed valgrind changes](https://github.com/rust-lang/rust/pull/24859)
* [Reviewed minor doc patch](https://github.com/rust-lang/rust/pull/25214)
* [Posted doc cleanup](https://github.com/rust-lang/rust/pull/25220)
* **[Posted std doc PR](https://github.com/rust-lang/rust/pull/25224)**
* [Thanked somebody for an experience report](https://users.rust-lang.org/t/first-experience-using-rust/1291)
* [Published Servo dep graph](http://www.reddit.com/r/rust/comments/35dh3e/servos_dependency_graph/)
* [Wrote cargo-lock-to-dot](https://github.com/brson/cargo-lock-to-dot)

# 2015-05-07

* Running multiple crater builds for acrichto and nmatsakis
* [Posted regression report for removal of vec addition](https://github.com/rust-lang/rust/pull/25157#issuecomment-99967389)
* Sent stickers to organizer in Boulder
* [Reviewed cargo patch](https://github.com/rust-lang/cargo/pull/1590)
* [Retried this patch for arielb1](https://github.com/rust-lang/rust/pull/25123)
* [Submitted Awesome Rust to reddit](http://www.reddit.com/r/rust/comments/357td7/awesome_rust_a_curated_list_of_awesome_rust_code/)
* [Sent niko a regression report](https://gist.github.com/brson/b81e3032b1838119e5ff)
* **Working on release notes**
* [Updated RELEASES.md and AUTHORS.txt](https://github.com/rust-lang/rust/pull/25196)
* [Posted message asking for audits of AUTHORS.txt](https://users.rust-lang.org/t/last-chance-for-immortality-audit-the-rust-1-0-authors-file/1278)
* [Reviewed pr backport](https://github.com/rust-lang/rust/pull/25192)
* [Re-reviewed valgrind configure change](https://github.com/rust-lang/rust/pull/24859)

# 2015-05-06

* Responded to Diane Tate about Rust communication ideas
* [Added Boulder to launch events](https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171/12)
* Interviewed candidate

# 2015-05-05

* [Reviewed some cargo pr](https://github.com/rust-lang/cargo/pull/1577#event-296920209)
* [Merged beta cherry-pick](https://github.com/rust-lang/rust/pull/25121)
* [Commented on multirust update bug](https://github.com/brson/multirust/issues/60#issuecomment-99163538)
* [Commented about nickel.rs](https://users.rust-lang.org/t/my-steps-with-rust-nickel-rs/1240/2?u=brson)
* Submitted ticket to ship t-shirts
* [Posted beta.4 update to www](https://github.com/rust-lang/rust-www/pull/119)
* [Posted minutes](https://internals.rust-lang.org/t/weekly-meetings-2015-05-05-servo-test-and-stability-irc-floods-semver/1994/1)
* [Posted comment about Option::unwrap_unchecked](https://github.com/rust-lang/rfcs/pull/1095#issuecomment-99237453)
* [Reviewed more windows path fixes](https://github.com/rust-lang/rust/pull/25134)
* [Published twir](http://www.reddit.com/r/rust/comments/34zx2h/this_week_in_rust_80/)
* [Reviewed another cargo patch](https://github.com/rust-lang/cargo/pull/1584#event-297733466)
* [Posted suggestions to read burntsushi's crates](http://www.reddit.com/r/rust/comments/3505gf/what_rust_repositories_would_you_recommend/cqzvpiq)

# 2015-05-04

* Sent email to Jason in mailroom coordinating t-shirt shipment tomorrow
* [Reviewed windows path fix](https://github.com/rust-lang/rust/pull/25103)
* [Filed PR to fix rustup.sh docs](https://github.com/rust-lang/rust/pull/25104)
* [Filed PR to fix rustup.sh docs on www](https://github.com/rust-lang/rust-www/pull/118)
* [Posted psa about rustup.sh changes](https://users.rust-lang.org/t/rustup-sh-no-longer-should-be-run-under-sudo/1233)
* Posted status update
* **Wrote more http database api in Rust**

# 2015-05-03

* **Refactoring crater javascript**
* **Running regression tests**
* [Posted regression report](https://internals.rust-lang.org/t/regression-report-beta-2015-05-01-vs-nightly-2015-05-03/1990)

# 2015-05-02

* [Reviewed rust-buildbot musl patch](https://github.com/rust-lang/rust-buildbot/pull/11)
* [Reviewed multirust typo](https://github.com/brson/multirust/pull/59)

# 2015-05-01

* [Merged rustup.sh interactive upgrade](https://github.com/rust-lang/rustup/pull/12)
* [Suggested building electron on servo](http://www.reddit.com/r/rust/comments/34jb2x/integration_of_a_rust_based_editor_into_servo/)
* [Upgraded multirust with rustup.sh](https://github.com/brson/multirust/commit/42159402a73d9aca32b517928457e8be79d1f239)
* [Further commenting on multirust dylib problems](https://github.com/brson/multirust/issues/43#issuecomment-98207119)
* [Filed PR with travis for updating rustup.sh](https://github.com/travis-ci/travis-build/pull/443)
* **Sorted t-shirts for shipping**
* [Triaged typecheck pr](https://github.com/rust-lang/rust/pull/25038)
* Discussed shipping logistics with Jason
* Invalidated nightlies in cdn

# 2015-04-30

* [**Modifying rustup.sh to not need sudo**](https://internals.rust-lang.org/t/what-changes-are-necessary-to-make-rustup-sh-handle-sudo-itself/1948/6?u=brson)
* [Closed beta target libs bug](https://github.com/rust-lang/rust/issues/24959)
* [Retriaged beta/stable CI bug](https://github.com/rust-lang/rust/issues/23757)
* [Retriaged cloudfront issue](https://github.com/rust-lang/rust/issues/21239)
* [Reviewed beta PR](https://github.com/rust-lang/rust/pull/25004)
* [Updated cargo pairing for 1.0](https://github.com/rust-lang/rust-packaging/pull/30)
* Started a new beta build
* [Filed issue to hash CFG_FILENAME_EXTRA](https://github.com/rust-lang/rust/issues/25007)
* [Gave mw link to some dbg issue](https://internals.rust-lang.org/t/weekly-meetings-2015-04-28-servo-podcasting-snapshots-transmute-static-assert/1974)
* [Filed PR for rustup upgrade](https://github.com/rust-lang/rustup/pull/12)
* [Made patch to update rust-www for rustup](https://github.com/brson/rust-www/tree/rustup)
* [Made patch to update rust for rustup](https://github.com/brson/rust/tree/rustup)
* [Reviewed rustdoc patch](https://github.com/rust-lang/rust/pull/24989)
* [Responded to u.rlo category thread](https://users.rust-lang.org/t/category-suggestions/659)
* [Reopened multirust rpath bug](https://github.com/brson/multirust/issues/43#event-293925562)
* [Merged multirust submodule patch](https://github.com/brson/multirust/pull/58)
* [Commented on problem with exe installers](https://github.com/rust-lang/rust/issues/24397)
* [Responded to issue about identical regions in errors](https://github.com/rust-lang/rust/issues/15391#issuecomment-97165966)
* [Posted minor doc fix](https://github.com/rust-lang/rust/pull/25020)
* [Filed request for Error for ()](https://github.com/rust-lang/rust/issues/25023)
* **Writing more HTTP server for crater**

# 2015-04-29

* [Reviewed cargo patch to add filtering to benchmarks](https://github.com/rust-lang/cargo/pull/1557)
* [Reviewed cargo file naming patch](https://github.com/rust-lang/cargo/pull/1558)
* [Reviewed cargo build script cfg patch](https://github.com/rust-lang/cargo/pull/1505)
* [Tagged multirust 0.0.5](https://github.com/brson/multirust/issues/57)
* Scheduled meeting to discuss meetup budget
* Reassigned P-backcompat-lang to P-high
* [Reviewed cargo timeout patch](https://github.com/rust-lang/cargo/pull/1564)
* [Reviewed another cargo patch](https://github.com/rust-lang/cargo/pull/1559)
* [Replied to doc url thread](http://www.reddit.com/r/rust/comments/34be9u/is_it_planned_to_support_docrustlangorgbeta_or/)
* Did t-shirt inventory
* **Allocating t-shirts to meetups**
* [Fixed discourse github login problem](https://users.rust-lang.org/t/problem-with-github-logins/1182)
* [Responded to felix's thread about --enable-debug](https://internals.rust-lang.org/t/why-is-enable-debug-implying-disable-optimize-for-rustc/1978)
* [Added some categories to users.rlo](https://users.rust-lang.org/t/category-suggestions/659/12)
* [Fixed betas containing all target libs](https://github.com/rust-lang/rust-buildbot/commit/165befeaa478efc30ba7f1433845f5bd4688e959)
* [Reviewed another cargo pr](https://github.com/rust-lang/cargo/pull/1566)

# 2015-04-28

* Conducted interview
* [Reviewed multirust doc PR](https://github.com/brson/multirust/pull/56)
* [Posted meeting minutes](https://internals.rust-lang.org/t/weekly-meetings-2015-04-28-servo-podcasting-snapshots-transmute-static-assert/1974)
* [Responded to AtomicPtr issue](https://users.rust-lang.org/t/how-to-share-atomicptr-t-between-threads/1166)
* [Posted list of launch meetups](https://users.rust-lang.org/t/a-list-of-rust-1-0-launch-meetups/1171)
* Writing crater-web

# 2015-04-27

* [Updated beta date on www](https://github.com/rust-lang/rust-www/commit/fcd298d84341b7299a8a69709cb38ff3dedecc00)
* [Thanked somebody for tips on Iron](http://users.rust-lang.org/t/example-of-building-a-web-service-and-client-with-iron-and-hyper/1095/6?u=brson)
* [Commented on missing libc decls](http://internals.rust-lang.org/t/lack-of-c99-integer-types-in-libc/1969/2?u=brson)
* [Posted TWiR](http://www.reddit.com/r/rust/comments/342ckb/this_week_in_rust_79/)
* Updated DNS for discourse https
* **Refactoring crater**
* Started building snaps for tamird
* **Creating crater docker image**
* [Merged twir PR](https://github.com/cmr/this-week-in-rust/pull/53)
* [Merged twir PR](https://github.com/cmr/this-week-in-rust/pull/54)
* Redeployed twir
* [Published updated regression report](https://gist.github.com/brson/7a9954d09898e1606958)
* Finished setting up https for users and internals

# 2015-04-26

* [Updated website for beta.3](https://github.com/rust-lang/rust-www/pull/116)
* **Writing twir**

# 2015-04-25

* [Published regression report](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-nightly-2015-04-24/1967)
* **Writing crater-db in Rust**

# 2015-04-24

* Interviewed devops candidate
* [Reviewed acrichto's musl patch](https://github.com/rust-lang/rust/pull/24777)
* [Reviewed minor test cleanup](https://github.com/rust-lang/rust/pull/24751)
* Entered Q2 goals in workday
* [Fixed beta fallout](https://github.com/rust-lang/rust/pull/24785)
* Retriaged old P-backcompat issues
* [Closed old issue about numeric impls](https://github.com/rust-lang/rust/issues/4955)

# 2015-04-23

* [Posted something encouraging about carboxyl](http://www.reddit.com/r/rust/comments/33ll59/interactive_2d_applications_with_carboxyl_and/)
* [Reviewed acrichto's blog post](https://github.com/rust-lang/blog.rust-lang.org/pull/51/files)
* [Reviewed acrichto's cherry-pick](https://github.com/rust-lang/rust/pull/24708)
* [Re-triaged scoped bug](https://github.com/rust-lang/rust/issues/24292)
* [Re-triaged rustc cli future-proofing](https://github.com/rust-lang/rust/issues/19051)
* [Complimented arielb1's sweet wins](https://github.com/rust-lang/rust/pull/24615#issuecomment-95743109)
* Told London organizers they have no budget
* [Asked Amsterdam organizer for shipping info](http://www.reddit.com/r/rust/comments/33lf0y/10_release_party_in_amsterdam/cqml0sm)
* [Posted update on launch events](http://users.rust-lang.org/t/rust-1-0-launch-event-details-action-required-for-event-organizers/1025/15?u=brson)
* [Responded to RustDT release](http://users.rust-lang.org/t/rustdt-0-2-0-released-auto-complete-with-racer/1109)

# 2015-04-22

* [Tried to get Gankro and nrc to work together on benchmarking](http://users.rust-lang.org/t/we-need-non-trivial-idiomatic-workloads-for-meaningful-performance-evaluation/493/4)
* [Posted PR remove version numbers from betas](https://github.com/rust-lang/rust/pull/24693)
* [Posted fix for rust-installer CDPATH problem](https://github.com/rust-lang/rust-installer/pull/32)
* [Posted fix for rustup CDPATH problem](https://github.com/rust-lang/rustup/pull/11)
* Fixed same problem in multirust
* Uploaded new rustup.sh
* [Upgraded rust-installer for rust](https://github.com/rust-lang/rust/pull/24704)
* [Upgraded rust-installer for cargo](https://github.com/rust-lang/cargo/pull/1543)
* [Upgraded rust-installer for rust-packaging](https://github.com/rust-lang/rust-packaging/pull/29)
* [Replied about discourse HTTPS](http://users.rust-lang.org/t/secure-access-via-https/296/11)
* [Replied about crater native deps](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-beta-2015-04-17/1931/3)
* [Fixed typo in valgrind patch](https://github.com/rust-lang/rust/pull/24477)
* [Posted investigation into Iron and Hyper](http://users.rust-lang.org/t/example-of-building-a-web-service-and-client-with-iron-and-hyper/1095)
* [Posted internals thread about sudo and rustup.sh](http://internals.rust-lang.org/t/what-changes-are-necessary-to-make-rustup-sh-handle-sudo-itself/1948)

# 2015-04-21

* [Commented on Yurume's Grisu patch](https://github.com/rust-lang/rust/pull/24612#issuecomment-94878903)
* [Merged some changes to the Rust wiki backup](https://github.com/rust-lang/rust-wiki-backup/pull/1)
* [Bumped version to 1.1](https://github.com/rust-lang/rust/pull/24670)
* [Reviewed TRPL index update](https://github.com/rust-lang/rust/pull/24669)
* Pulled together some CloudFront download metrics for Jack
* [Filed an issue to store overrides in override directories](https://github.com/brson/multirust/issues/54)
* [Filed issue for multirust docs](https://github.com/brson/multirust/issues/55)
* [Posted PR to Lars ICFP paper](https://github.com/larsbergstrom/papers/pull/33)
* [Posted doc about branching](http://internals.rust-lang.org/t/release-channels-git-branching-and-the-release-process/1940)
* [Posted minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-04-21-servo-licensing-iter-overflow-float-formatting-branching-for-betas/1941)
* [Merged twir PR and deployed](https://github.com/cmr/this-week-in-rust/pull/52)
* [Responded to question about branching](http://www.reddit.com/r/rust/comments/33edbz/release_channels_git_branching_and_the_release/cqk562u)
* [Closed shallow submodule PR](https://github.com/rust-lang/rust/pull/24521)

# 2015-04-20

* [Responded to multirust bug report about remove-toolchain](https://github.com/brson/multirust/issues/53)
* Asked Neil Lalande to set up https for discourse.
* [Thanked SimonSapin for updating Travis to use rustup.sh](http://users.rust-lang.org/t/psa-1-0-0-beta-2-is-out/1019/8)
* [Reviewed deprecation removal](https://github.com/rust-lang/rust/pull/24636)
* [Sent steveklabnik counter-edit for branching strategy](https://gist.github.com/brson/789281140c4672af4a44)
* [Reviewed doc patch](https://github.com/rust-lang/rust/pull/24640)
* [Reviewed fixes to rustup](https://github.com/rust-lang/rustup/pull/4)
* [Reviewed gpg fix to rustup](https://github.com/rust-lang/rustup/pull/6)
* [Commented on ldconfig problem](https://github.com/rust-lang/rust/issues/24358)
* [Fixed ldconfig issue in rustup](https://github.com/rust-lang/rustup/pull/9)
* [Closed ldconfig issue](https://github.com/rust-lang/rust/issues/24358)
* [Closed ldconfig issue](https://github.com/rust-lang/rustup/issues/3)
* [Closed ldconfig issue](https://github.com/rust-lang/rust/pull/24529)
* [Posted patch making stability attributes an error](https://github.com/rust-lang/rust/pull/24646)
* [Fixed multirust problem deleting toolchains](https://github.com/brson/multirust/issues/53)
* [Commented about making mem::forget safe](https://github.com/rust-lang/rfcs/pull/1066#issuecomment-94599639)
* Triaged t-shirts
* [**Published new TWiR**](http://this-week-in-rust.org/blog/2015/04/20/this-week-in-rust-78/)
* [Posted TWiR to /r/rust](http://www.reddit.com/r/rust/comments/33b4gp/this_week_in_rust_78/)
* [Updated TWiR based on feedback](http://www.reddit.com/r/rust/comments/33b4gp/this_week_in_rust_78/cqj86xl)

# 2015-04-18

* **Got crater running on aws**
* [Merged removal of 30-minute intro from www](https://github.com/rust-lang/rust-www/pull/114)
* [Reviewed removal of 30-minute intrto](https://github.com/rust-lang/rust/pull/24572)
* **Investigating writing parts of crater in Rust**
* [Posted regression report](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-beta-2015-04-17/1931)

# 2015-04-17

* **Working on launch event details**
* Booked whistler tickets
* [Bumped prerelease to .3](https://github.com/rust-lang/rust/pull/24532)
* [Posted website update for beta.2](https://github.com/rust-lang/rust-www/pull/113)
* [Updated 1.0 cargo rev again](https://github.com/rust-lang/rust-packaging/pull/28)
* [Commented on 'nix' renaming issue](https://github.com/carllerche/nix-rust/issues/76)
* [Commented on ldconfig problem](https://github.com/rust-lang/rust/pull/24529)
* [Commented on rust-www style issue](https://github.com/rust-lang/rust-www/pull/102)
* [Merged icon updates](https://github.com/rust-lang/rust-www/pull/106)
* [Added header anchors to blog.rlo](https://github.com/rust-lang/blog.rust-lang.org/commit/527256b93f43526bdbe6ac518398d198e1278d06)
* [Thanked WindowsBunny for starting windows discussion](https://github.com/rust-lang/rfcs/issues/1061)
* [Commented on rust-installer issue with cd and CDPATH](https://github.com/rust-lang/rust-installer/issues/31)
* [Posted notet about beta.2](http://users.rust-lang.org/t/psa-1-0-0-beta-2-is-out/1019)
* [Fixed typos to beta psa](http://users.rust-lang.org/t/psa-1-0-0-beta-2-is-out/1019)
* [Reviewed parallel make fixes](https://github.com/rust-lang/rust/pull/24537)
* Fixed beta doc links on website
* [Bumped TWiR thread](http://users.rust-lang.org/t/twir-quote-of-the-week/328/36)
* Sent internal email about party status updates
* Sent email asking for t-shirt earmarks
* [Made thread about launch events](http://users.rust-lang.org/t/rust-1-0-launch-event-details-action-required-for-event-organizers/1025)
* [Commented on beta changelog](http://www.reddit.com/r/rust/comments/32yahe/psa_100beta2_is_out/cqg7lv9)

# 2015-04-16

* In SF for interviews
* [Updated cargo rev for beta](https://github.com/rust-lang/rust-packaging/pull/27)
* [Responded to question about crater sandboxing](http://www.reddit.com/r/rust/comments/32rvwv/regression_report_leveraging_cratesio_to/)
* Started a build of 1.0.0-beta.2
* [Reviewed cargo version bump](https://github.com/rust-lang/cargo/pull/1531#issuecomment-93858895)
* Responded to various threads about release parties
* Interviewed devops candidate

# 2015-04-15

* Returned old laptop
* [Submitted fix for configure not detecting missing valgrind](https://github.com/rust-lang/rust/pull/24477)
* Reviewed aturon's governance RFC draft
* **Continuing patch to disable is/us**
* [Posted patch for is/us removal](https://github.com/rust-lang/rust/pull/24485)
* [Filed bug reports about annoying breakage of browser.ctrlTab.previews](https://bugs.launchpad.net/ubuntu/+source/firefox/+bug/1444788)
* [Filed bug reports about annoying breakage of browser.ctrlTab.previews](https://bugzilla.mozilla.org/show_bug.cgi?id=1154922)

# 2015-04-14

* Pushed updates to TWiR
* [Commented on effort to improve error messages](https://www.reddit.com/r/rust/comments/32jdq9/help_write_rust_error_explanations/cqc6q42)
* [Closed multirust issue about show-override](https://github.com/brson/multirust/issues/28)
* [Closed shasum multirust issue](https://github.com/brson/multirust/issues/39)
* [Responded to felix about the rust rev used to build cargo](https://github.com/brson/multirust/issues/43)
* [Updated stability PR for long lines](https://github.com/rust-lang/rust/pull/24399)
* Asked #it for help resetting yichoi's irc password
* [Commented on adding a new linking mode](https://github.com/rust-lang/rust/pull/24369#issuecomment-93016637)
* [Posted meeting minitues](http://internals.rust-lang.org/t/weekly-meetings-2015-04-14-beta-symlinks-missing-stdio-handles-compound-assignment-rustdoc-cleanup-coc-enforcement/1884)
* Scheduled meeting w/ bhearsum to talk about migrating to taskcluster
* Sent Rainer a 3D Rust logo
* [Responded to homebrew issue](https://github.com/brson/multirust/issues/8#issuecomment-92025028)

# 2015-04-13

* [Released rustup 0.0.3](http://users.rust-lang.org/t/multirust-0-0-3/954)
* [Responded to thread about using the rust logo](https://www.reddit.com/r/rust/comments/32e8i5/rust_logo_ok_to_customise_and_use/cqb0p55)
* Sent check for the cake
* [Closed rustup issue](https://github.com/rust-lang/rust/issues/21149)
* [Closed issue about --test not working on beta](https://github.com/rust-lang/rust/issues/20832)
* [Found ricky26's msvc branch to work off of](https://github.com/ricky26/rust/tree/msvc)
* [Reviewed adding rbe to doc index](https://github.com/rust-lang/rust/pull/24393)
* Responded to bangalore meetup group about reimbursement
* [Reviewed doc PR](https://github.com/rust-lang/rust/pull/24395)
* [Replied to old bug about missing videos](https://bugzilla.mozilla.org/show_bug.cgi?id=1003414)
* Uploaded rustup.sh
* [Reviewed minor visitor change](https://github.com/rust-lang/rust/pull/24391)
* [**Posted TWiR**](https://www.reddit.com/r/rust/comments/32ii3z/this_week_in_rust_77/).
* [Posted patch to make stability attributes a warning](https://github.com/rust-lang/rust/pull/24399).
* [Merged minor rust-installer fix](https://github.com/rust-lang/rust-installer/pull/28).
* [Triaged issue of rustup failing on travis](https://github.com/rust-lang/rustup/issues/3).

# 2015-04-12

* Made multirust's blastoff script interactive
* Making a beta build to test problems with --test
* **TWiR**
* [Filed issue about multirust's use of CARGO_HOME](https://github.com/brson/multirust/issues/49)
* Worked on the multirust 0.0.3 release

# 2015-04-11

* [Filed PR for improvements to rustup](https://github.com/rust-lang/rustup/pull/1)
* [Reviewed issue about playpen breakage](https://github.com/rust-lang/rust-www/issues/107)
* [Merged playpen version fix](https://github.com/rust-lang/rust-www/pull/110)
* [Investigated multirust cargo build dylib issues](https://github.com/brson/multirust/issues/43#issuecomment-91953336)
* [Investidaget multirust symlink issue](https://github.com/brson/multirust/issues/40)
* **Fixing multirust bugs**

# 2015-04-10

* [Created rustup repo](https://github.com/rust-lang/rustup)
* [Removed rustup.sh from repo](https://github.com/rust-lang/rust/pull/24285)
* [Bumped prerelease version](https://github.com/rust-lang/rust/pull/24287)
* [Posted message about rustup.sh rewrite](http://users.rust-lang.org/t/rustup-sh-rewritten/913)
* [Reviewed deriving fix](https://github.com/rust-lang/rust/pull/24270)
* [Published regression report](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-nightly-2015-04-10/1860)
* Asked erickt for list of people hosting launch events
* **Finishing up features and polish for rustup.sh**
* **Rebuild multirust off of rustup.sh**
* [Reviewed rust-www playpen error handling improvements](https://github.com/rust-lang/rust-www/pull/108)
* [Replied to regression report discrepencies](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-nightly-2015-04-10/1860/3)

# 2015-04-09

* **Continuing rustup.sh upgrades**
* [Made nightly turn on LLVM asserts](https://github.com/rust-lang/rust/pull/24205)
* Testing a patch for pitdicker
* [Reviewed error message improvement](https://github.com/rust-lang/rust/pull/24242)
* [Closed old squiggle PR](https://github.com/rust-lang/rust/pull/21639)
* [Closed --test RFC](https://github.com/rust-lang/rfcs/pull/345)
* [Closed RFC about concat_bytes!](https://github.com/rust-lang/rfcs/pull/566)
* Communicating with the baker
* Signed up for whistler
* [Filed multirust issue](https://github.com/brson/multirust/issues/47)
* [Commented on multirust update all](https://github.com/brson/multirust/issues/46)

# 2015-04-08

* Starting crate tests for https://github.com/rust-lang/rust/pull/23905
* **Working on turning off debug asserts https://github.com/rust-lang/rust/issues/17081**
* Posted note to meetup group about cameras recording Thursday http://www.meetup.com/Rust-Bay-Area/events/220627544/
* [Posted config optimization patch](https://github.com/rust-lang/rust/pull/24205)
* [Reviewed cargo docopt patch](https://github.com/rust-lang/cargo/pull/1499)
* [Reviewed extension of plugins to llvm passes](https://github.com/rust-lang/rust/pull/24207)
* [Reviewed changes to windows sockets](https://github.com/rust-lang/rust/pull/24211#issuecomment-91064248)
* Reimaged rust-android1 so nrc can benchmark on it
* [Encouraged balisong](https://www.reddit.com/r/rust_gamedev/comments/31v165/progress_report_on_balisong_voxel_renderer/)
* **Working on merging rustup.sh with multirust**
* [Expressed opinion about need for knowing `rustc`](http://internals.rust-lang.org/t/newcomer-to-rust-my-experience/1816/48)
* [Praised hoverbear's Raft](https://www.reddit.com/r/rust/comments/31ynnz/raft_small_status_update/cq68ydu)

# 2015-04-07

* [Praised Racer updates](https://www.reddit.com/r/rust/comments/31pwpa/racer_progress_update_5_cargo_support/cq45acx)
* **TWiR**
* Testing crates against nightly to run a regression report
* [Reopened issue about parsing match exprs](https://github.com/rust-lang/rust/issues/22546#issuecomment-90607706)
* [Reopened issue about rustup.sh upgrade](https://github.com/rust-lang/rust/issues/21149)
* [Assigned disabling debug asserts to me](https://github.com/rust-lang/rust/issues/22390)
* [Pinged aturon on conversion RFC update](https://github.com/rust-lang/rfcs/pull/1016#issuecomment-90651591)
* [Commented on builder guidelines RFC](https://github.com/rust-lang/rfcs/pull/1036#issuecomment-90653620)
* [Commented on dtor double-panics](http://users.rust-lang.org/t/should-destructors-panic-should-they-double-panic/847/7)
* [Responded to q from felix about rust-www example length](http://users.rust-lang.org/t/is-the-homepage-example-wrong-on-purpose/763/11)
* [Merged defaulting rust-www to offer msi installer](https://github.com/rust-lang/rust-www/pull/105).
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-04-07-beta-prs-abs-rustdoc-and-std-facade-wiki-docs-out-of-tree/1838).
* [Reviewed RUSTUP_PREFIX](https://github.com/rust-lang/rust/pull/24171).
* [Published TWiR](http://this-week-in-rust.org/blog/2015/04/06/this-week-in-rust-76/).
* Resume triage.
* Asked Richard to move airmo broadcast from 8 to 7 PM Thursday.
* Ordered pizza for meetup.
* Submitted expense report for meetup.
* Sent enquiries about cakes.
* [Posted call for Rust-related media](http://users.rust-lang.org/t/call-for-rust-related-photos-and-media/878).

# 2015-04-06

* Talking with people about Rust party decorations
* [Reviewed README patch to multirust](https://github.com/brson/multirust/pull/44)
* [Triaged multirust issue](https://github.com/brson/multirust/issues/35)
* [Filed issue about not downloading docs](https://github.com/rust-lang/rust/issues/24117)
* [Commented on issues about multirust man pages](https://github.com/brson/multirust/issues/24)
* [Merged compiler fence RFC](https://github.com/rust-lang/rfcs/pull/888)
* [Closed multiple 'if let' RFC](https://github.com/rust-lang/rfcs/pull/937#issuecomment-90170630)
* [Postponed type level numerics RFC](https://github.com/rust-lang/rfcs/pull/884)
* [Closed generic over & types RFC](https://github.com/rust-lang/rfcs/pull/976#issuecomment-90175633)
* [Reviewed acrichto's new fs RFC](https://github.com/alexcrichton/rfcs/blob/f3153e669c807e844f82bf361fdb6dd106eb8d49/text/0000-io-fs-2.1.md)
* [Submitted C++ comparison to r/rust](https://www.reddit.com/r/rust/comments/31nl8y/more_rust_compared_to_c/)
* [Filed issue about bug in license headers](https://github.com/rust-lang/rust/issues/24122)
* Running more crater tests against beta
* **TWiR**
* [Posted toolchain report for beta](https://gist.github.com/brson/5ac7c009252f5b1916ec)
* Sent summary of party info thus far

# 2015-04-05

* Wrote a quarterly self-evaluation
* [Praised ecr's work](https://www.reddit.com/r/rust/comments/31kq70/patina_a_formalization_of_the_rust_programming/cq2jjeg)
* Finished reviewing book

# 2015-04-04

* [Responded to GC q](https://www.reddit.com/r/rust/comments/31fnsy/rust_and_gc/)
* [Commented about multithreaded println](https://github.com/rust-lang/rust/pull/24029#issuecomment-89624649)
* [Reviewed a diagnostics patch](https://github.com/rust-lang/rust/pull/24072)

# 2015-04-03

* Still sick
* Ran more crate tests against beta
* Responded to emails
* [Responded to multirust feature request for per-branch overrides](https://github.com/brson/multirust/issues/41)
* [Responded to multirust bug report](https://github.com/brson/multirust/issues/40)
* [Responded to issue about detecting stable compilers](https://www.reddit.com/r/rust/comments/31ca6u/what_to_do_with_a_crate_that_still_uses_unstable/cq0aosg)
* [Responded to issue about smaller installs](https://www.reddit.com/r/rust/comments/31cmn8/rustupsh_minimal/)
* Working on rust cakes
* [Responded to issue about distro bootstrapping](https://www.reddit.com/r/programming/comments/31btd8/rust_100_beta_is_here/cq0i6dg)
* [Triaged a minor doc bug](https://github.com/rust-lang/rust/pull/24048)
* Talked to manish about CI improvements

# 2015-04-02

* Still sick
* Responded to Gerv about Rust security properties
* Submitted requests for 5/15 release party reservations
* [Upgraded rust-installer for rust](https://github.com/rust-lang/rust/pull/23978)
* [Upgraded rust-installer for rust-packaging](https://github.com/rust-lang/rust-packaging/pull/26)
* Upgraded rust-installer for multirust
* [Upgraded rust-installer for cargo](https://github.com/rust-lang/cargo/pull/1476)
* Sent some thoughts about the release party
* [Put together list of crates that work on stable](https://www.reddit.com/r/rust/comments/319y87/some_crates_that_work_with_100beta/)

# 2015-03-29

* Sick again today
* [Merged rust-packaging fixes](https://github.com/rust-lang/rust-packaging/pull/25)
* Reviewed ch 7 of packt book
* [Reviewed rust-installer fix for OS X](https://github.com/rust-lang/rust-installer/pull/28)
* [Reviewed rust-installer option parsing fix](https://github.com/rust-lang/rust-installer/pull/29)
* Pushed another test build to beta
* [Reviewed minor Cargo fix](https://github.com/rust-lang/cargo/pull/1455#event-269006350)
* [Reviewed guidelines fix](https://github.com/rust-lang/rust-guidelines/pull/47#issuecomment-87860263)

# 2015-03-28

* [Supported Manish's effort to focus on polish](http://internals.rust-lang.org/t/proposal-spend-most-of-the-beta-period-not-writing-new-features/1770)
* **Finished support for custom builds in crater**
* Sent niko a regression report for his coherence fix

# 2015-03-27

* **Rust builds on taskcluster almost working**
* Reviewing automation needs for interviews
* [Posted PR to feature gate slice patterns](https://github.com/rust-lang/rust/pull/23794)
* Sent email to mw upon contract end
* [Reviewed minor build system fix](https://github.com/rust-lang/rust/pull/23678)
* [Posted build results for crates against beta](http://users.rust-lang.org/t/heres-the-top-200-crates-built-against-the-beta-channel/757)
* [Triaged doc PR](https://github.com/rust-lang/rust/pull/23795)
* [Responded to niko's coherence thread](http://internals.rust-lang.org/t/rebalancing-coherence-potential-late-breaking-change/1763/4)
* [Reviewed removal of extern crate string syntax](https://github.com/rust-lang/rust/pull/23786)
* **Continuing work on rustup**
* [Posted a comparison report between beta and nightly](https://gist.github.com/brson/19996f5d0c35ba059042)
* [Solicited TWiR quotes](http://users.rust-lang.org/t/twir-quote-of-the-week/328/32)
* [Reviewed curl fix](https://github.com/rust-lang/rust/pull/23332)
* [Reviewed read_into_string/read_into_vec](https://github.com/rust-lang/rust/pull/23335)
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/23746)
* [Praised mw's work](https://www.reddit.com/r/rust/comments/30icbp/use_rustgdb_and_rustlldb_for_improved_debugging/)

# 2015-03-26

* **Debugging taskcluster because it keeps failing to build rust**
* **Started upgrading rustup.sh**
* [Reviewed doc update](https://github.com/rust-lang/rust/pull/23746)
* [Filed issue about beta/stable integration](https://github.com/rust-lang/rust/issues/23757)
* [Reviewed should_fail patch](https://github.com/rust-lang/rust/pull/23752)
* [Triaged doc PR](https://github.com/rust-lang/rust/pull/23758)
* [Fixed --help in rust-installer](https://github.com/rust-lang/rust-installer/pull/27)
* [Upgraded rust-installer for rust](https://github.com/rust-lang/rust/pull/23763)
* [Upgraded rust-installer for cargo](https://github.com/rust-lang/cargo/pull/1461)
* [Upgraded rust-installer for rust-packaging](https://github.com/rust-lang/rust-packaging/pull/24)
* [Reviewed removal of deprecated -l form](https://github.com/rust-lang/rust/pull/23765)
* **Working on feature gating slice patterns**

# 2015-03-25

* Still trying to get custom builds to work in taskcluster-crater
* [Posted PSA about feature staging](http://users.rust-lang.org/t/psa-feature-staging-will-be-fully-activated-in-the-next-nightly/731)
* Starting another beta build
* [Cross-posted servo steam rendering to /r/rust](https://www.reddit.com/r/rust/comments/30blul/the_steam_store_rendered_in_servo/)

# 2015-03-24

* [Reviewed adjustments to crate hyphenation](https://github.com/rust-lang/rust/pull/23546)
* [Reviewed cargo adjustments to crate hyphenation](https://github.com/rust-lang/cargo/pull/1443)
* [Replied to thread about forum HTTPS](http://users.rust-lang.org/t/secure-access-via-https/296/11)
* [Posted twir](https://www.reddit.com/r/rust/comments/305l66/this_week_in_rust_75/)
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/48)
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/49)
* Pushed a branch to try for nagisa.
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-03-24-fott-unsafe-destructor-drop-order-libtest/1752)
* Pushed test build to beta.
* [Updated rust-packaging for beta](https://github.com/rust-lang/rust-packaging/commit/046c2809a1b64b36123144c613f4f2b378164ef1)
* Set up OpSec account on AWS for Gene Wood.

# 2015-03-23

* [Rebasing gate PR](https://github.com/rust-lang/rust/pull/23598)
* [Rebased rpath fix](https://github.com/rust-lang/rust/pull/23283)
* [Merged windows packaging improvements](https://github.com/rust-lang/rust-packaging/pull/23)
* [Fixed 32-bit userspace detection](https://github.com/rust-lang/rust/pull/23650)
* [Commented on mahkoh's stupid not-a-systems-language RFC](https://github.com/rust-lang/rfcs/pull/1007/#issuecomment-85209677)
* [r-'d adding a stable 'try' function](https://github.com/rust-lang/rust/pull/23651)
* [Filed bug to create Rust video](https://bugzilla.mozilla.org/show_bug.cgi?id=1146672)
* [Reviewed cargo refactor](https://github.com/rust-lang/cargo/pull/1441)
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/23645)

# 2015-03-22

* Spent more time tweaking crater and rebasing gating patch

# 2015-03-21

* **Rebasing feature gating patch**
* **Setting up new computer**
* [Finally submitted PR for new gating patch](https://github.com/rust-lang/rust/pull/23598)

# 2015-03-20

(Didn't record anything)

# 2015-03-19

* **Expanding taskcluster-crater to report on arbitrary Rust revisions**
* **More rebasing of feature gating patch...**
* Started email thread about 1.0 promotional video

# 2015-03-18

* Asked for money to move mac builders to macstadium.com
* [Replied to burntsushi's q's about our builders' specs](https://www.reddit.com/r/rust/comments/2zf8ur/rust_infrastructure_can_be_your_infrastructure/cpj1o0n)
* **Again working on crater**
* **Again rebasing feature gating patch**
* Responded to Lars about hiring issues

# 2015-03-17

* **Yet more work on staging - getting doctests to pass**
* **Yet more work on crater reporting**
* [Moved fott archives to my blog](https://brson.github.io/fott.html)
* [Responded to q about whether a LICENSE file is necessary](https://github.com/contain-rs/compare/pull/11#issuecomment-82617344)
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-03-17-checked-overflow-and-casts-hyphens-in-crate-names/1728/1)
* [Reviewed socket fix](https://github.com/rust-lang/rust/pull/23450)
* Reviewed Niko's blog post on future feature categorization

# 2015-03-16

* [Replied to users.rlo categeries thread](http://users.rust-lang.org/t/category-suggestions/659/7?u=brson)
* [Nominated borrowck regression](https://github.com/rust-lang/rust/issues/23338)
* Helped osa1 find debugger test command interepreter in compiletest
* **Working on today's twir**
* **Continuing to work on feature staging patch**
* Submitted status report
* [Filed tracking issue for closure return type tweak](https://github.com/rust-lang/rust/issues/23420)
* [Merged closure return type RFC](https://github.com/rust-lang/rfcs/pull/968)
* [Posted TWiR](https://www.reddit.com/r/rust/comments/2z9u9v/this_week_in_rust_74/)
* [Posted rust-dev archives](http://users.rust-lang.org/t/rust-dev-archives/662)
* Responded to ecoop thread
* Started thread about Rust 1.0 release party
* Followed up with mw about employment stuff

# 2015-03-15

* More work on crater reports
* More work on feature staging

# 2015-03-14

* Rebasing feature gating patch
* [Worked on brson.github.io a bit](http://brson.github.io)
* [Filed issue about multirust/rusti incompatibility](https://github.com/brson/multirust/issues/38)
* **Working on crater reporting**
* [Complimented a rust game](https://www.reddit.com/r/rust_gamedev/comments/2z01t0/phage_a_completed_7day_roguelike_in_rust/)

# 2015-03-13

* [Praised 'a swift guide to rust'](https://www.reddit.com/r/rust/comments/2yvqa1/a_swift_guide_to_rust/cpdv7h7)
* [Responded to q about linux runtime deps](https://www.reddit.com/r/rust/comments/2ytxr1/rust_and_runtime_dependencies/cpdve3p)
* **Working on feature staging**
* Added korean meetup to calendar
* [Replied to core-team style guide thread](http://internals.rust-lang.org/t/style-guide/1721)
* Solicited news for twir
* [Replied to renaming SocketAddress](https://github.com/rust-lang/rfcs/pull/923#issuecomment-79244464)
* [Rerequested a copy of the rust-dev archive](https://bugzilla.mozilla.org/show_bug.cgi?id=1124794)
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/46)
* [Merged twir fix](https://github.com/cmr/this-week-in-rust/pull/47)

# 2015-03-12

* Rescheduled core team meeting
* **Fixing rustdoc so doc tests can contain crate attributes**
* **Writing report generation code for taskcluster-crater**
* [Closed PR to remove --help -v](https://github.com/rust-lang/rust/pull/22495)
* [Filed issue about extern crate in fns](https://github.com/rust-lang/rust/issues/23314)
* [Replied to q about runtime deps](https://www.reddit.com/r/rust/comments/2ytxr1/rust_and_runtime_dependencies/cpcwmjo)
* [Reviewed .msi gui updates](https://github.com/rust-lang/rust-packaging/pull/21)
* [Replied to a question about Arc<Mutex>](http://users.rust-lang.org/t/concurrency-and-mutex-lock-with-thread-spawns/618/3)
* [Reviewed to multirust to fix gdb](https://github.com/brson/multirust/pull/37)
* [Replied to thread about bad syntax highlighting on forums](http://users.rust-lang.org/t/syntax-highlighting/620/2)
* [Fixed beta naming again](https://github.com/rust-lang/rust/pull/23325)
* [Reviewed FreeBSD build fix](https://github.com/rust-lang/rust/pull/23324)
* [Reviewed jemalloc annotations](https://github.com/rust-lang/rust/pull/23322)
* [Reviewed rustup instructions patch](https://github.com/rust-lang/rust/pull/23315)
* [Reviewed rust-installer fix for /bin/echo not existing](https://github.com/rust-lang/rust-installer/pull/26)
* [Triage](https://github.com/rust-lang/rust/issues/23286)
* [Reviewed rustdoc ICE fix](https://github.com/rust-lang/rust/pull/23328)
* [Re-reviewed allocator attribute](https://github.com/rust-lang/rust/pull/23322)
* [Reviewed doc cleanup](https://github.com/rust-lang/rust/pull/23329)

# 2015-03-11

* Sent bstrie the picture of him with the Rust logo in SF.
* [Merged TWiR fix](https://github.com/cmr/this-week-in-rust/pull/45)
* [Merged TWiR fix](https://github.com/cmr/this-week-in-rust/pull/43)
* [Submitted PR to fix -C rpath regression](https://github.com/rust-lang/rust/pull/23283)
* [Submitted issue about Path::relative_from](https://github.com/rust-lang/rust/issues/23284)
* Had lunch with security candidate (Julia) and Daniel Veditz
* Investigating 16 hours of buildbot not running.
* [Submittid a logging improvement to homu](https://github.com/barosl/homu/pull/67)
* Shipped stickers to denmark
* [Updated servo PR](https://github.com/servo/servo/pull/5026)
* [Reviewed closure syntax RFC](https://github.com/rust-lang/rfcs/pull/968)
* [Reviewed discriminant_value intrinsic RFC](https://github.com/rust-lang/rfcs/pull/639)
* [Reviewed generic conversian RFC](https://github.com/rust-lang/rfcs/pull/529)
* [Reviewed iter::once RFC](https://github.com/rust-lang/rfcs/pull/771)
* [Reviewed empty structs RFC](https://github.com/rust-lang/rfcs/pull/218)
* [Reviewed must_use RFC](https://github.com/rust-lang/rfcs/pull/886)
* [Reviewed octal lexing RFC](https://github.com/rust-lang/rfcs/pull/879)
* [Reviewed stdio RFC](https://github.com/rust-lang/rfcs/pull/899)
* **Continuing work on making undeclared features errors**
* **Continuing work on scheduling and reporting for taskcluster-crater**
* [Reviewed 64-bit msi support](https://github.com/rust-lang/rust-packaging/pull/17)
* [Reviewed --no-combined patch for rust-packaging](https://github.com/rust-lang/rust-packaging/pull/18)
* [Commented on fence RFC](https://github.com/rust-lang/rfcs/pull/888)
* [Commented on RFC about multiple 'if let'](https://github.com/rust-lang/rfcs/pull/937#issuecomment-78404051)
* [Commentod on RFC about numerics in types](https://github.com/rust-lang/rfcs/pull/884#issuecomment-78404331)
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/23295)

# 2015-03-10

* Took PTO today.
* Replied to an event organizer about stickers and setting up the calendar
* [Responded to cargo make install issue](https://github.com/rust-lang/cargo/issues/1399#issuecomment-78112007)
* Respnoded to nrc about ECOOP participation
* [Gave dhuseby password to buildbot](https://github.com/rust-lang/rust-buildbot/pull/10#issuecomment-78115602)
* [Encouraged adding --depth=1 to submodule checkout](https://github.com/rust-lang/rust/issues/23258#issuecomment-78117493)
* [**Working on rpath regression**](https://github.com/rust-lang/rust/issues/23140)
* [Reviewed minor stability patch](https://github.com/rust-lang/rust/pull/23263)
* **Working on TWiR**
* [Crossposted piston-on-emscripten story](https://www.reddit.com/r/rust/comments/2ylzkx/a_game_written_with_piston_running_on_the_web_via/)
* [Reviewed rustdoc optimization](https://github.com/rust-lang/rust/pull/22769)
* [Posted TWiR](https://www.reddit.com/r/rust/comments/2ymb4t/this_week_in_rust_73/)
* [Reviewed loop-statement RFC](https://github.com/rust-lang/rfcs/pull/955#issuecomment-78177092)
* [Reviewed at_exit restoration](https://github.com/rust-lang/rust/pull/23267)

# 2015-03-09

* **Working on patch for undeclared features**
* Wrote status update
* [**Investigating rpath regression**](https://github.com/rust-lang/rust/issues/23140)
* **Adding scheduling logic to taskcluster-crater**
* [Reviewed cargo profiling tweaks](https://github.com/rust-lang/cargo/pull/1395)
* [Commented about removing unnecessary bounds from types](https://github.com/rust-lang/rust/pull/23176#issuecomment-77963160)
* [Reviewed log refactoring PR](https://github.com/rust-lang/log/pull/27)
* [Reviewed mpsc Error impl](https://github.com/rust-lang/rust/pull/23125)
* [Reviewed wrapping patch for isaac](https://github.com/rust-lang/rust/pull/23105)
* [PR triage](https://github.com/rust-lang/rust/pull/23028#issuecomment-77965359)
* [Reviewed rust-installer PR](https://github.com/rust-lang/rust-installer/pull/25#issuecomment-77967127)
* [Reviewed bloat optimizations](https://github.com/rust-lang/rust/pull/22948#issuecomment-77968425)
* [Reviewed man page cleanup](https://github.com/rust-lang/rust/pull/22832)
* [Promised to review a --libdir PR](https://github.com/rust-lang/rust/pull/22831#issuecomment-77968736)
* [Reviewed OpenBSD fixes for rust-installer](https://github.com/rust-lang/rust-installer/pull/23)

# 2015-03-08

* Mork work on brson.github.io
* Reviewing Packt book ch 5

# 2015-03-07

* **Writing about locks**
* Working on brson.github.io

# 2015-03-06

* **Working on making undeclared features errors**
* [Removed two old green threading tests](https://github.com/rust-lang/rust/pull/23124)
* [Reviewed cargo PR](https://github.com/rust-lang/cargo/pull/1387)
* Followed up on bstrie community space access
* [Filed PR to not implement RFC 8 - intrinsics](https://github.com/rust-lang/rfcs/pull/948)
* Rescheduled meetings for DST
* [Begged for mor twir quotes](http://users.rust-lang.org/t/twir-quote-of-the-week/328/15)
* **Working on taskcluster-crater**

# 2015-03-05

* Talked to acharles about rustfmt
* [Reviewed minor doc patch](https://github.com/rust-lang/rust/pull/23048)
* [Created thread about rustfmt](http://internals.rust-lang.org/t/completing-rustfmt-and-the-rust-style-guidelines/1685)
* Attended triage
* [Responded to tessel thread](https://www.reddit.com/r/rust/comments/2y2enz/new_microcontroller_that_aims_to_support_rust_as/cp5movq)
* [Uploaded bitrig snap](http://internals.rust-lang.org/t/community-supported-build-slaves-for-2nd-tier-platforms/1528/11?u=brson)
* [Encouraged TWiR QOTWs](http://users.rust-lang.org/t/twir-quote-of-the-week/328/14)
* **Started patch to make undeclared features an error**
* [Renamed beta artifacts to 'beta'](https://github.com/rust-lang/rust/pull/23094)
* [Reviewed some script fixes](https://github.com/rust-lang/rust/pull/22474)
* Requested 3/17 airmo event cancelled.
* Scheduled room and airmo for 4/9.
* **More work on crater.**
* [Removed ignore-fast directives from test suite](https://github.com/rust-lang/rust/pull/23098)
* [Removed unused test](https://github.com/rust-lang/rust/pull/23099)
* [Reviewed cargo doc PR](https://github.com/rust-lang/cargo/pull/1365)
* [Reviewed minor doc fix](https://github.com/rust-lang/rust/pull/23100)
* [Commented on Deref conventions](https://github.com/rust-lang/rfcs/issues/279#issuecomment-77495003)

# 2015-03-04

* In SF, mostly worked on crater

# 2015-03-03

* [**Added task scheduling and message monitoring to taskcluster-crater**](https://github.com/brson/taskcluster-crater)
* [Reviewed unicode fix](https://github.com/rust-lang/rust/pull/23000)
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-03-03-fott-filling-drop-type-ascription/1677)
* [Posted fott](https://github.com/rust-lang/rust/wiki/Doc-friends-of-the-tree#2015-03-03-manish-goregaokar-manishearth)
* [Reviewed liblibc fixes](https://github.com/rust-lang/rust/pull/23004)
* [Filed suggestion about multirust --which](https://github.com/brson/multirust/issues/36)
* [Closed old taskcluster-crater PR](https://github.com/jhford/taskcluster-crater/pull/1)
* [Posted new taskcluster-crater PR](https://github.com/jhford/taskcluster-crater/pull/2)
* [Closed tuple iteration RFC](https://github.com/rust-lang/rfcs/pull/870)
* Helped pnkfelix with accessing the infrastructure
* Responded to potential GSoC student
* Shipped stickers to mbrubeck
* [Reviewed unicode fix again](https://github.com/rust-lang/rust/pull/23000)
* [Reviewed tidy improvements](https://github.com/rust-lang/rust/pull/22029)
* [Promised dhuseby to upload his bitrig snap](http://internals.rust-lang.org/t/community-supported-build-slaves-for-2nd-tier-platforms/1528/9)
* Responded to another GSoC candidate
* Rehabbing stabworld to get some new info about popular crates and their feature usage

# 2015-03-02

* Wrote status update.
* [Published heka-rs](https://github.com/brson/heka-rs)
* Attended a number of meetings today.
* **Familiarized myself with taskcluster**
* [Updated num audit PR](https://github.com/rust-lang/rust/pull/22600)
* [Rebased servo cleanup PR](https://github.com/servo/servo/pull/5026)

# 2015-03-01

* Complimented Alexis on the std::collections docs
* [Fireworked default trait PR from flaper87](https://github.com/rust-lang/rust/pull/21689#issuecomment-76636425)
* **Working on TWiR**
* [Thanked kballard for I/O fixes](https://github.com/rust-lang/rust/pull/22749#issuecomment-76638381)
* Deleted 400GB of cores from mac3
* [Bumped by TWiR quote of the week thread](http://users.rust-lang.org/t/twir-quote-of-the-week/328/10)

# 2015-02-28

* [Filed issue about str docs](https://www.reddit.com/r/rust/comments/2xggd2/where_are_the_docs_for_str/)
* [Replied to __morestack_addr thread](http://users.rust-lang.org/t/why-is-my-rust-broken-26-02-2015-build/492/5)
* [Thanked dabaross for tip about rustup](http://internals.rust-lang.org/t/rsync-for-nightly-builds/1624/7)
* [Posted TWiR](https://www.reddit.com/r/rust/comments/2xmacv/this_week_in_rust_72/)

# 2015-02-27

* [Asked trink if I can publish his heka-rs prototype](https://www.reddit.com/r/rust/comments/2x5p96/would_mozilla_rewrite_heka_in_rust/coz60xo)
* Thanked csorenson for my new laptop
* [Made minor edits to servo paper](https://github.com/larsbergstrom/papers/pull/9)
* Minor work on taskcluster-crater
* [Reviewed steve's new ownership material](http://words.steveklabnik.com/a-new-introduction-to-rust)
* **Editing Lars's ICFP Servo paper**
* [Responded to thread about __morestack_addr](http://users.rust-lang.org/t/why-is-my-rust-broken-26-02-2015-build/492/2)
* [Cleaned up Rust section of servo paper](https://github.com/larsbergstrom/papers/pull/21)
* [Wrote about traits for servo paper](https://github.com/larsbergstrom/papers/pull/25)

# 2015-02-26

* [Responded to stability attrs on non-staged crates](https://github.com/rust-lang/rust/issues/22830)
* [Sent nagisa a fix for s3-directory-listing](https://github.com/nagisa/s3-directory-listing/pull/1)
* Deployed json and txt indexing for dist archives
* Helped gbs get latest cargo nightlies
* [Reviewed doc fix](https://github.com/rust-lang/rust/pull/22848)
* [Replied to core bloat thread](http://users.rust-lang.org/t/fixed-overhead-rust-bootloader-and-core-panicking/429/8)
* [Reviewed fs close](https://github.com/rust-lang/rust/pull/22849)
* [**Added release channel code to taskcluster-crater**](https://github.com/jhford/taskcluster-crater/pull/1)
* [Replied to heka.rs thread](https://www.reddit.com/r/rust/comments/2x5p96/would_mozilla_rewrite_heka_in_rust/)
* [Merged rust-cocoa fixes](https://github.com/servo/rust-cocoa/pull/76)
* [Replied to packaging policy](https://github.com/rust-lang/rust-packaging/issues/16#issuecomment-76327127)
* [Filed issue about creating .debs](https://github.com/rust-lang/rust-packaging/issues/19)
* [Filed issue about moving packaging into rust-installer](https://github.com/rust-lang/rust-packaging/issues/20)

# 2015-02-25

* [Responded to q about rt rust](https://www.reddit.com/r/rust/comments/2x0h17/whats_your_killer_rust_feature/cowz2y2)
* [Posted list of Rust IRC channels](http://users.rust-lang.org/t/a-list-of-rust-irc-channels/472)
* **Working on multiple buildbot fixes still**
* [Updated meeting minutes](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-02-24.md)
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-02-24-should-fail-irc-error-codes-type-ascription-triage/1659)
* Created new Windows AMI with new build of dojob-rs
* [Responded to servo PR feedback](https://critic.hoppipolla.co.uk/showcomment?chain=10648)
* [Responded to multirust no-doc issue](https://github.com/brson/multirust/issues/33)
* [Tested patch to fix who can kill builds](https://github.com/rust-lang/rust-buildbot/pull/6)
* [Responded to thread about heka.rs](https://www.reddit.com/r/rust/comments/2x5p96/would_mozilla_rewrite_heka_in_rust/)
* [Merged fix for killing builds in rust-buildbot](https://github.com/rust-lang/rust-buildbot/pull/9)
* [Merged fix for rust-buildbot permissions](https://github.com/rust-lang/rust-buildbot/pull/6)
* Deployed new rust-buildbot to production
* Restarted mac3
* [Closed desync issue](https://github.com/rust-lang/rust-buildbot/issues/7)
* Restarted mac4
* Changed pw of 'rust' buildbot user
* [Reviewed guidelines patch](https://github.com/rust-lang/rust-guidelines/pull/44)
* [Responded to code bloat thread](http://users.rust-lang.org/t/fixed-overhead-rust-bootloader-and-core-panicking/429/5)
* [Pinged alex on io critique](http://internals.rust-lang.org/t/issues-in-new-i-o/1658/3)
* Replied to Alex about missing mac slaves
* Sent response to GSoC inquiry

# 2015-02-24

* [Told SimonSapin how --version build date is determined](https://www.reddit.com/r/rust/comments/2wy8sr/this_week_in_rust_71/)
* [Reviewed twir fix](http://envisage-project.eu/proving-android-java-and-python-sorting-algorithm-is-broken-and-how-to-fix-it/)
* [Reviewed twir fix](https://github.com/cmr/this-week-in-rust/pull/42)
* [Reviewed twir fix](https://github.com/cmr/this-week-in-rust/pull/40)
* Redeployed twir
* [Posted status update](http://benjamin.smedbergs.us/weekly-updates.fcgi/user/banderson%40mozilla.com)
* [Commented on nightly failures](https://www.reddit.com/r/rust/comments/2wy8sr/this_week_in_rust_71/covtic8)
* Doing a variety of buildbot upgrades
* [Promised pnkfelix and niko will solve a bug that affects gfx-rs and piston](https://github.com/rust-lang/rust/issues/22536#issuecomment-75880833)
* [Responded to thread about rt rust](https://www.reddit.com/r/rust/comments/2x0h17/whats_your_killer_rust_feature/cow5kt7)
* **Working buildbot fixes, adding dojob-rs on windows**
* [Responded to thread about core::panicking bloat](http://users.rust-lang.org/t/fixed-overhead-rust-bootloader-and-core-panicking/429/2)
* [Responded to thread about removed runtime features](https://www.reddit.com/r/rust/comments/2x2pon/what_significant_languageruntime_features_were/)

# 2015-02-23

* [Replied to nrc's thread about rustc APIs](https://www.reddit.com/r/rust/comments/2wwqco/a_tutorial_on_creating_a_dropin_replacement_for/)
* [Posted another quote](http://users.rust-lang.org/t/twir-quote-of-the-week/328/8)
* [Responded to NetBeans thread](https://www.reddit.com/r/rust/comments/2wqyip/rust_netbeans_plugin_embeds_native_rust_compiler/cov2z1j)
* [Responded to q about grep](https://github.com/rust-lang/rust/pull/22474)
* [Published twir](http://this-week-in-rust.org/blog/2015/02/23/this-week-in-rust-71/)
* [Linked twir from r/rust](https://www.reddit.com/r/rust/comments/2wy8sr/this_week_in_rust_71/)

# 2015-02-22

* Wrote down new rust project ideas.
* Poking at MSP430 support for rustc, needed by Contiki OS.
* [Posted update to servo cleanup PR](https://github.com/servo/servo/pull/5026)
* [Gave encouraging comment about netbeans plugin](https://www.reddit.com/r/rust/comments/2wqyip/rust_netbeans_plugin_embeds_native_rust_compiler/)
* [Replied to rsync nightlies thread](http://internals.rust-lang.org/t/rsync-for-nightly-builds/1624/3)

# 2015-02-21

* [Dealing with lots of integration failures on windows](https://github.com/rust-lang/rust/issues/22628)
* [Attempted to hack around the problem of stdtest hanging](https://github.com/rust-lang/rust-buildbot/commit/9911005f4df1695a6f09142c02eced4ecf04a218)
* [Did some servo cleanup](https://github.com/servo/servo/pull/5013)
* [Wrote rustle for installing Cargo apps](https://github.com/brson/rustle)
* [Replied to comment on HN about non-rusties installing Rust apps](https://news.ycombinator.com/item?id=9088331)
* [Posted rustle to HN](https://www.reddit.com/r/rust/comments/2wqgb4/rustle_install_cargo_applications_without/)
* Restarted mac6
* [Posted encouraging comments about exa](https://www.reddit.com/r/rust/comments/2wp3pp/ive_added_loads_more_features_to_exa_my_ls/)
* [Reviewed typo](https://github.com/rust-lang/rust/pull/22657)
* Spent way too much time getting Rust to compile to msp430

# 2015-02-20

* [Reviewed iter::once RFC](https://github.com/rust-lang/rfcs/pull/771)
* Tested alpha.2 installers
* [Posted cleaned up explanation of static dispatch](https://github.com/rust-lang/rust/pull/22593)
* Tagged alpha.2 release
* [Merged alpha.2 website updates](https://github.com/rust-lang/rust-www/pull/97)
* [Merged alpha.2 blog post](https://github.com/rust-lang/blog.rust-lang.org/pull/25)
* [Merged alpha.2 blog patch](https://github.com/rust-lang/blog.rust-lang.org/pull/26)
* Pushed wiki changes for alpha.2
* [Posted support for rust-media](https://www.reddit.com/r/rust/comments/2wl5cl/rustmedia_a_portable_media_player_framework_for/)
* [Posted core::num audit](https://github.com/rust-lang/rust/pull/22600)
* [Retried bitrig PR which is hitting timeouts](https://github.com/rust-lang/rust/pull/21959)
* Replied privately to dhuseby about bitrig PR
* [Reviewed twir PR](https://github.com/cmr/this-week-in-rust/pull/36)
* [Rejected twir PR to clear up twir numbering ambiguity](https://github.com/cmr/this-week-in-rust/pull/32)
* Starting on twir
* [Responded to discourse https issue](http://users.rust-lang.org/t/secure-access-via-https/296/6)
* Responded by email to newbie looking to contribute
* [Reviewed fixes to blog post](https://github.com/rust-lang/blog.rust-lang.org/pull/29)
* [Reviewed more fixes to blog post](https://github.com/rust-lang/blog.rust-lang.org/pull/30#issuecomment-75335153)
* [And more](https://github.com/rust-lang/blog.rust-lang.org/pull/31)
* [Commented on issue of macro internal functions](https://github.com/rust-lang/rust/issues/22607)
* [Outlined a plan for supporting arbitrary Cargo's in multirust](https://github.com/brson/multirust/issues/27)
* [Reviewed test case PR](https://github.com/rust-lang/rust/pull/22620)
* Worked on some servo cleanup and commenting

# 2015-02-19

* Started beta build
* [Fixed a bug in the beta windows logic in rust-packaging](https://github.com/rust-lang/rust-packaging/commit/1147e42a997ffe7b667fc032631b62fb9f283fd8)
* Attended triage
* [Reviewed more msi changes](https://github.com/rust-lang/rust-packaging/pull/14)
* Responded to jhford about global ci questions
* Sent Manish credentials to any-build on buildbot so he can kill
  builds until I fix the bug that makes 'rust' unable to kill builds.
* [Merged PR to add .msi downloads to rust-www](https://github.com/rust-lang/rust-www/pull/96)
* Auditing core::num for ints
* [Updated rust-www for alpha.2](https://github.com/rust-lang/rust-www/pull/97)
* Updated wiki for alpha.2
* Reading TRPL
* [Closed issue about sudo considered harmful](https://github.com/rust-lang/rust-www/pull/76)
* [Filed bug about trpl introduction of ownership and lifetimes](https://github.com/rust-lang/rust/issues/22553)
* [Triaged ld incompatibility with Java](https://github.com/rust-lang/rust/issues/22528)
* [Made some minor updates to book about strings](https://github.com/rust-lang/rust/pull/22556)
* [Merged RFC to not put impls next to structs](https://github.com/rust-lang/rfcs/pull/735)

# 2015-02-18

* [Updated rust-packaging for correct alpha.2 cargo rev](https://github.com/rust-lang/rust-packaging/commit/fef2d016ab79f5b36a8b25832446f643fa0f6020)
* Started a dry run of alpha.2
* Finally got nightly building again
* [Updated README cleanup PR](https://github.com/rust-lang/rust/pull/22395)
* Bumped tomorrow's meetup guestlist to 80
* Ordered pizza for tomorrow's meetup
* Expensed pizza tomorrow's meetup
* [Wrote new readme for installer](https://github.com/rust-lang/rust-packaging/pull/13)
* [Reviewed vadim's fixes for msi](https://github.com/rust-lang/rust-packaging/pull/12)
* [Posted PSA about broken travis builds](http://users.rust-lang.org/t/psa-travis-builds-with-language-rust-are-broken/389)
* [Offered opinion on Int::pow](https://github.com/rust-lang/rust/pull/22087#issuecomment-74970211)
* [Deferred to aturon on deprecating int modules](https://github.com/rust-lang/rust/pull/22503)
* [Reviewed some cleanup](https://github.com/rust-lang/rust/pull/22509)
* [Created a patch to make rust work on travis again](https://github.com/travis-ci/travis-build/pull/391)
* [Updated travis PSA thread with patch link](http://users.rust-lang.org/t/psa-travis-builds-with-language-rust-are-broken/389/2)
* [Expressed enthusiasm for njn's bloom filter wins](https://github.com/servo/servo/pull/4938)
* [Made relnotes PR](https://github.com/rust-lang/rust/pull/22517)
* [Reviewed `str` docs](https://github.com/rust-lang/rust/pull/22513)

# 2015-02-17

* [Responded to DroidLogician about office visits](https://www.reddit.com/r/rust/comments/2w6rrk/im_going_to_be_in_sf_in_april_for_a_convention_is/cooewtk)
* [Filed issue about security policy](https://github.com/rust-lang/rust/issues/22464)
* Responded to a person looking to contribute to Rust
* Working on problem with CentOS linux builders checking out deep submodules
* Recompiled git from master on the CentOS linux-snap builders
* Reimaged linux-snap builders
* Updated slave-list.txt on prod and dev build masters
* acrichto volunteered to restart buildmaster
* Started build of nightlies with new installer
* Started build of nightly combined package with new installer
* [Commented on recycling slaves](https://github.com/rust-lang/rust/issues/22448)
* [Posted meeting minutes](http://internals.rust-lang.org/t/weekly-meetings-2015-02-17-fott-security-bugs-code-completion-open-ended-proposals-struct-syntax-alpha2-integer-suffixes-overflow/1608/1)
* [Posted TWiR](https://www.reddit.com/r/rust/comments/2waarz/this_week_in_rust_70/)
* [Merged TWiR fix and republished](https://github.com/cmr/this-week-in-rust/pull/39)

# 2015-02-16

* Gave michaelwoerister r+ for debuginfo
* [Reviewed string matching RFC](https://github.com/rust-lang/rfcs/pull/528)
* [Reviewed comment conventions RFC](https://github.com/rust-lang/rfcs/pull/505)
* [Reviewed hash RFC](https://github.com/rust-lang/rfcs/pull/823)
* [Reviewed rustup locale fix](https://github.com/rust-lang/rust/pull/22420)
* Working on TWiR
* [Responded to thread about 2nd tier builders](http://internals.rust-lang.org/t/community-supported-build-slaves-for-2nd-tier-platforms/1528/7)
* [Postponed unsafe enum RFC](https://github.com/rust-lang/rfcs/pull/724)
* [Enthused about acrichto's fixes of stability lints](https://github.com/rust-lang/rust/pull/22127)

# 2015-02-15

* [Made encouraging comments about Iron perf](https://www.reddit.com/r/rust/comments/2vzxjr/poor_http_performance_iron_framework/comjwyx)
* Looked for TWiR stories
* [Responded to concerns about bootstrapping Rust with Cargo](http://internals.rust-lang.org/t/should-cargo-be-written-in-rust/1595/4)
* Thanked Lars for making faster builds of Rust for Servo
* [Made some cleanups to the source README](https://github.com/rust-lang/rust/pull/22395)
* [Reviewed kmc's macro docs](https://github.com/rust-lang/rust/pull/22393/files)
* [Wrote encouraging things about dotdosh's miscompile fix](https://github.com/rust-lang/rust/pull/22385)
* [Reviewed fhahn's solution to compiletest capturing output incorrectly](https://github.com/rust-lang/rust/pull/22371)
* [Reviewed rustc_attrs feature](https://github.com/rust-lang/rust/pull/22336)
* [Approved Cargo rust-installer upgrade](https://github.com/rust-lang/cargo/pull/1301)
* [Merged rust-packaging upgrade](https://github.com/rust-lang/rust-packaging/pull/11)
* [Reviewed comment patch about Vec::from_iter](https://github.com/rust-lang/rust/pull/22394)
* [Reviewed stevek's CONTRIBUTING patches](https://github.com/rust-lang/rust/pull/22282)
* [Responded to GSoC thread](http://www.reddit.com/r/rust/comments/2vy5n3/rust_in_gsoc_2015/com8cq3)
* [Added rustfmt to list of gsoc projects](https://wiki.mozilla.org/Community:SummerOfCode15:Brainstorming)
* [Expressed mild support for publishing std via cargo](http://internals.rust-lang.org/t/std-on-crates-io/1585/5?u=brson)

# 2015-02-14

* [Praised Trace Quest Season 5](https://www.reddit.com/r/rust/comments/2vva9y/trace_quest_5_season_1_results/colj77q)
* [Responded to a Q about linking std to musl](https://news.ycombinator.com/item?id=9050828)
* [Commented on Huon's expansion of `must_use` to `ok`](https://github.com/rust-lang/rust/pull/22348)
* [Spent time working on int audit](https://github.com/rust-lang/rust/issues/22240)
* [Reviewed rename of std::failure](https://github.com/rust-lang/rust/pull/22347)
* [Filed uint -> usize PR](https://github.com/rust-lang/rust/pull/22350)
* [Revised usize PR](https://github.com/rust-lang/rust/pull/22350)
* [Stared a node.js lib for interacting with Rust infra](https://github.com/brson/rustworld)

# 2015-02-13

* Interviewed an intern candidate
* [Replied to coroutine thread](http://users.rust-lang.org/t/goroutine-coroutine-or-the-similar-in-rust/327/2)
* [Replied to most coveted features thread](http://users.rust-lang.org/t/most-coveted-rust-features/324/4)
* [Begged people for quotes of the week for TWiR](http://users.rust-lang.org/t/twir-quote-of-the-week/328)
* [Responded to publishing std on crates.io](http://internals.rust-lang.org/t/std-on-crates-io/1585/2)
* [Pushed temporary disable of msi packaging](https://github.com/rust-lang/rust-packaging/commit/c183da100c025d80f685970ac0e6ee21e5852a8d)
* Triggered rebuild of last night's failed nightlies
* [Updated version number for alpha.2](https://github.com/rust-lang/rust/pull/22292)
* Registered event, room, and airmo for 3/17 meetup
* [Reviewed doc PR](https://github.com/rust-lang/rust/pull/22293)
* [Re-reviewed bitrig PR](https://github.com/rust-lang/rust/pull/21959)
* [Investigated dist desync issue](https://github.com/rust-lang/rust-buildbot/issues/7)
* [Revised Rust --version PR](https://github.com/rust-lang/rust/pull/22201)
* [Revised Cargo --version PR](https://github.com/rust-lang/cargo/pull/1292)
* Attempting to debug wix failures on windows bots
* [Reviewed fhahn's fixes for early termination errors](https://github.com/rust-lang/rust/pull/22117)
* [Reviewed fhahn's parse-fail tests](https://github.com/rust-lang/rust/pull/22118)
* [Requested retry of PR](https://github.com/rust-lang/rust/pull/21376)
* [Reviewed test](https://github.com/rust-lang/rust/pull/22302)
* [Reviewed int audit of std::failure](https://github.com/rust-lang/rust/pull/22303)
* [Filed issue about incorrect naming in std::failure](https://github.com/rust-lang/rust/issues/22306)
* [Expressed my disapproval of another stealth attribute addition](https://github.com/rust-lang/rust/pull/22278#issuecomment-74342434)
* [Reviewed grammar docs](https://github.com/rust-lang/rust/pull/22308)
* Responded to jhford about global CI design
* [Submitted rustup PR for suruga](https://github.com/klutzy/suruga/pull/7)

# 2015-02-12

* [Complimented RustAudio](https://www.reddit.com/r/rust/comments/2vn0xx/rustaudio_a_collection_of_crates_for_audio_and/)
* [Emailed the person from Codius with words of encouragement](https://codius.org/blog/codius-rust/)
* [Responded to Process not Send on Windows thread](https://www.reddit.com/r/rust/comments/2vmzmh/coremarkersend_difference_mac_vs_windows/)
* [Promised to look into HTTPS on discourse](http://users.rust-lang.org/t/secure-access-via-https/296)
* Asked Neil from Discourse about HTTPS
* [Responded to OS X 10.5 q](http://users.rust-lang.org/t/rust-on-mac-os-10-5-8/304/3)
* [Responded to thread about platform-specific APIs in std](http://internals.rust-lang.org/t/what-rate-of-importability-do-we-allow-in-libstd/1556)
* Attended Rust triage
* Triaged a few I-compiletime issues
* [Reviewed ammendments to object safety RFC](https://github.com/rust-lang/rfcs/pull/817)
* Got OS X .pkg working with component selection and uninstallation
* Got .exe and .msi working with new installer
* Created new EC2 AMI for windows with WIX
* Added new windows AMI's to buildbot config. Waiting to restart buildbot.
* [Updated multirust for rust-installer](https://github.com/brson/multirust/commit/095e540026824e0e047f7325b9fcd5959fb9aeec)
* [Closed old build system issue](https://github.com/rust-lang/rust/issues/12363)
* [Added a --without flag to rust-installer](https://github.com/rust-lang/rust-installer/commit/60fd8abfcae50629a3fc664bd809238fed039617)
* [Filed multirust issue about slow blastoff script](https://github.com/brson/multirust/issues/29)
* [Filed PR for Rust rust-installer upgrade](https://github.com/rust-lang/rust/pull/22256)
* [Filed PR for Cargo rust-installer upgrade](https://github.com/rust-lang/cargo/pull/1301)
* [Filed PR for rust-packaging rust-installer upgrade](https://github.com/rust-lang/rust-packaging/pull/11)
* [Updated rust-buildbot to enable .msi](https://github.com/rust-lang/rust-buildbot/commit/ff9022c9c8be047152d5cf01a22a6b5408738a4e)
* Restarted buildbot for .msi
* [Updated .msi issue](https://github.com/rust-lang/rust/issues/21118#issuecomment-74205873)
* [Tightened up description of 1.0.0-alpha on rust-www](https://github.com/rust-lang/rust-www/pull/95)
* [Filed issue about unsafety of Repr](https://github.com/rust-lang/rust/issues/22260)

# 2015-02-11

* [Responded to questions about no_std gating](https://www.reddit.com/r/rust/comments/2vc05d/this_week_in_rust_69/coibg28)
* Obsoleted --disable-verify flag to rust-installer
* Improved error handling in rust-installer
* Upgraded rust-installer for rust, cargo, rust-packaging, multirust
* [Posted rust-installer upgrades](https://github.com/rust-lang/rust-installer/pull/20)
* [Updated rust-installer readme](https://github.com/rust-lang/rust-installer/commit/5bd23c2845a11856f6bc14b61c9ea963656731d9)
* Pointed out the Samsung OSG position to some people
* Responded to dhuseby about bitrig PR
* Responded to Packt telling them I'll review their book
* [Voiced approval for from_elem RFC](https://github.com/rust-lang/rfcs/pull/832#issuecomment-73989469)
* Submitted expense report for users.rust-lang.org
* Pinged jhford about global CI
* [Posted PR to solve --version date confusion](https://github.com/rust-lang/rust/pull/22201)
* [Likewise a Cargo PR for --version](https://github.com/rust-lang/cargo/pull/1292)

# 2015-02-10

* Wrote today's fott
* Weekly meeting
* [Expressed my opinion about array pattern simplification](https://github.com/rust-lang/rfcs/pull/495)
* [Posted meeting minutes](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-02-10.md)
* [Posted meeting minute thread](http://internals.rust-lang.org/t/weekly-meetings-2015-02-10-fott-unsafe-no-drop-flag-box-rfc-fallout-unused-attribute-feature-gate-sizeof-alignof-array-patterns/1578)
* [Re-reviewed bitrig PR](https://github.com/rust-lang/rust/pull/21959)
* [Reviewed fix for ffi ICEs](https://github.com/rust-lang/rust/pull/22160)
* Cleaned up rust-installer and made its backups work consistently

# 2015-02-09

* [Posted twir](https://www.reddit.com/r/rust/comments/2vc05d/this_week_in_rust_69/)
* [Filled out status report](http://benjamin.smedbergs.us/weekly-updates.fcgi/user/banderson%40mozilla.com)
* [Replied to thread about coding standards](https://www.reddit.com/r/rust/comments/2vc63z/with_10_around_the_corner_is_there_a_place_to/)
* [Responded to some multirust issue](https://github.com/brson/multirust/issues/27)
* [Responded to pnkfelix about RFC features](https://github.com/rust-lang/rfcs/pull/815)
* More work on rust-installer

# 2015-02-08

* twir
* [Asked for rebase of bitrig patch](https://github.com/rust-lang/rust/pull/21959)
* [Reviewed test tweaks](https://github.com/rust-lang/rust/pull/21862)

# 2015-02-07

* [Filed a bug about debugging extern fns](https://github.com/rust-lang/rust/issues/22071)
* Working on OpenSSL impl

# 2015-02-06

* [Responded to 1.0 bugs issue on internals](http://internals.rust-lang.org/t/a-list-of-high-priority-but-borderline-1-0-bugs-that-need-attention/1550/5?u=brson)
* [Commented on rustc --version confusion](https://github.com/rust-lang/rust/issues/21957#issuecomment-73290701)
* [Cheerlead parse-fail test PR](https://github.com/rust-lang/rust/pull/22011#issuecomment-73291027)
* [Reviewed featureck bugfix PR](https://github.com/rust-lang/rust/pull/21994)
* [Reviewed debuginfo fixes](https://github.com/rust-lang/rust/pull/21970)
* [Gisted some things that surprised me about assoc types](https://gist.github.com/brson/32a56693fa554b5a9dda)
* [Reviewed Cargo style PR](https://github.com/rust-lang/cargo/pull/1278)
* [Reviewed Cargo bugfix](https://github.com/rust-lang/cargo/pull/1271)
* [Reviewed another Cargo PR](https://github.com/rust-lang/cargo/pull/1270)
* [Reviewed another Cargo PR](https://github.com/rust-lang/cargo/pull/1269)
* Working on more rust-installer features
* [Responded to weird installation error](https://github.com/rust-lang/rust-buildbot/issues/7#issuecomment-73328834)
* [Reviewed kmc's plugin change](https://github.com/rust-lang/rust/pull/22026)

# 2015-02-05

* Attended triage
* [Reviewed bitrig patch](https://github.com/rust-lang/rust/pull/21959)
* [Added feature header to RFC template](https://github.com/rust-lang/rfcs/pull/815)
* [Revised my tidy PR](https://github.com/rust-lang/rust/pull/21619)
* [Filed issue about package desync](https://github.com/rust-lang/rust-buildbot/issues/7)
* [Added version file to combined package](https://github.com/rust-lang/rust-packaging/commit/d3ef82109a2894d5472a5a7b7b093b9bf9d9f3bc)
* [Posted list of high-pri bugs](http://internals.rust-lang.org/t/a-list-of-high-priority-but-borderline-1-0-bugs-that-need-attention/1550)
* [Reviewed binop optimization](https://github.com/rust-lang/rust/pull/21985)
* [Opened thread about porting maintenance](http://internals.rust-lang.org/t/rust-platform-portability-and-maintenance/1551)
* [Responded to RandomAccessIterator pre-RFC](http://internals.rust-lang.org/t/pre-rfc-remove-randomaccessiterator/1548)
* [Responded to Aatch's type system refactoring post](http://internals.rust-lang.org/t/type-system-refactoring/1487/7)
* [Re-reviewed RangeFull PR](https://github.com/rust-lang/rust/pull/21947)
* Upgraded rust-installer metadata version
* [Cheerleadered a windows fix in libpnet](https://github.com/libpnet/libpnet/pull/41)

# 2015-02-04

* [Reviewed bitflags PR](https://github.com/rust-lang/bitflags/pull/3)
* Cleaned up test harness in rust-installer and multirust
* Continued working on stable feature lint
* [Reviewed `..` syntax PR](https://github.com/rust-lang/rust/pull/21947)
* [Fixed printing errors to stderr in rust-installer](https://github.com/rust-lang/rust-installer/issues/5)
* [Fixed allcaps style issues in rust-installer](https://github.com/rust-lang/rust-installer/issues/1)
* [Responded to cross-compile thread](https://www.reddit.com/r/rust/comments/2ut7qi/rust_how_good_is_the_support_of_cross_compilation/)
* [Expressed reservation about timing of type ascription RFC](https://github.com/rust-lang/rfcs/pull/803#issuecomment-72975225)
* Reviewed various RFCs
* [rust-installer cleanup](https://github.com/rust-lang/rust-installer/pull/17)
* [Responded to SimonSapin's q about cargo isolation](https://github.com/brson/multirust/issues/25)
* [Filed issue about confusing date in rustc --version](https://github.com/rust-lang/rust/issues/21957)
* [Responded to confusing inference error](http://internals.rust-lang.org/t/compile-time-error-unable-to-infer-enough-type-information-about--/1396/7)
* [Filed issue about upgrading multirust independently](https://github.com/brson/multirust/issues/27)
* [Filed issue re Cargo testing vs. old rustc](https://github.com/rust-lang/cargo/issues/1276)
* [Filed issue re multirust show-toolchain](https://github.com/brson/multirust/issues/28)
* [Opend PR for stable_features lint](https://github.com/rust-lang/rust/pull/21958)
* [Building miniservo-gtk](https://github.com/glennw/miniservo-gtk)

# 2015-02-03

* Posted thread about community build slaves http://internals.rust-lang.org/t/community-supported-build-slaves-for-2nd-tier-platforms/1528
* Worked with Manish to identify bugs in stability attributes
* Triaged https://github.com/rust-lang/rust/issues/21905
* Reviewed https://github.com/rust-lang/rust-buildbot/pull/6
* Wrote readme for rust-buildbot
* Added a bitrig builder to buildbot
* Fixed windows lock problem by setting max builds to 1 on dist builders
* Followed up on cargo DMCA takedown https://bugzilla.mozilla.org/show_bug.cgi?id=1118571
* Sent huseby connection info for the bitrig1 slave
* Reviewed https://github.com/rust-lang/rust-by-example/pull/443
* Closed https://github.com/brson/multirust/issues/16

# 2015-02-02

* TWiR
* Gave Lonnen Alex and my email to add to heroku

# 2015-02-01

* Wrote patch for https://github.com/brson/multirust/issues/10
* Filed UI suggestion for crates.io
* Filed out-of-tree stability bug https://github.com/rust-lang/rust/issues/21858

# 2015-01-31

* Writing emails and drama cleanup

# 2015-01-30

* Deleted broken aws vpc config at direction of Dave Curado
* Replied to huon about meetup expenses
* Posted more multiline highlighting options https://github.com/rust-lang/rust/pull/21639
* Reviewed https://github.com/rust-lang/rust/pull/21509
* Posted patch to make unused_features warn by default https://github.com/rust-lang/rust/pull/21800

# 2015-01-29

* Made snaps https://github.com/rust-lang/rust/pull/21760
* Added a link to users.rust-lang.org to website
* Added links to users.rust-lang.org https://github.com/rust-lang/rust/pull/21762
* Reviewed https://github.com/rust-lang/rust-by-example/pull/438
* Posted req for help about discourse email topics https://meta.discourse.org/t/creating-topics-by-email-to-users-rust-lang-org-or-internals-rust-lang-org-fails-with-email-issue-unknown-to-address/24595
* Sent another email to jhford about global ci
* Reviewed https://github.com/rust-lang/cargo/pull/1243
* Reviewed https://github.com/rust-lang/cargo/pull/1250
* Redirected discuss.rust-lang.org to internals.rust-lang.org

# 2015-01-28

* Posted feature staging info http://discuss.rust-lang.org/t/psa-important-info-about-rustcs-new-feature-staging/82
* Reviewed https://github.com/cmr/this-week-in-rust/pull/31
* Hooked applicative event coordinator up with Niko, jdm, steve, jack
* Responded to http://discuss.rust-lang.org/t/psa-important-info-about-rustcs-new-feature-staging/82
* Reviewed https://github.com/rust-lang/rust/pull/21739

# 2015-01-27

* Rebased https://github.com/rust-lang/rust/pull/21248
* Updated clang toolchain on mac3 and put it back into rotation
* Posted meeting minutes
* Responded to strcat rants https://github.com/rust-lang/rfcs/pull/741
* Finished yesterday's twir
* Finished moving discuss.rust-lang.org to internals.rust-lang.org
* Filed PR to fix docs for internals.rust-lang.org https://github.com/rust-lang/rust/pull/21708

# 2015-01-26

* Reviewed https://github.com/rust-lang/rust/pull/21666
* Reviewed https://github.com/rust-lang/rust/pull/21643
* Filed bug to update rustdoc for feature staging https://github.com/rust-lang/rust/issues/21674
* Triaged https://github.com/rust-lang/rust/issues/19278

# 2015-01-25

* More rebasing feature staging
* Posted error squiggly patch https://github.com/rust-lang/rust/pull/21639

# 2015-01-24

* Closed https://github.com/rust-lang/rust/issues/9875
* Responded to HN thread https://news.ycombinator.com/item?id=8940590
* Reviewed https://github.com/rust-lang/rust/pull/21598

# 2015-01-23

* Reviewing and fixing multirust patch
* More slogging through feature staging
* Responded to service now question about the auto-responder message for rust-dev
* Set up DNS for internals.rust-lang.org
* Fixed a bug in beta naming https://github.com/rust-lang/rust/pull/21579

# 2015-01-22

* Updated calender with some events
* Filed emacs comment wrapping bug https://github.com/rust-lang/rust/issues/21524
* Merged https://github.com/rust-lang/rfcs/pull/587
* Send ml shutdown announcement
* Sent Jeff Atwood email asking for new discourse instance
* Reviewed https://github.com/rust-lang/rust/pull/21458
* Reviewed https://github.com/rust-lang/rfcs/pull/575
* Reviewed https://github.com/brson/multirust/pull/21
* Reviewed https://github.com/rust-lang/rust/pull/21540
* Reviewed https://github.com/rust-lang/rust/pull/21556

# 2015-01-21

* Found a bug in crates registry https://github.com/rust-lang/crates.io-index/issues/1
* Asked IT to send automated message to people sending messages to rust-dev
* Filed issue to remove #[prelude_import] https://github.com/rust-lang/rust/issues/21490
* Reviewed https://github.com/rust-lang/cargo/pull/1205
* Reviewed https://github.com/rust-lang/rust/pull/21481
* Reviewed https://github.com/rust-lang/rust/pull/21489
* Continuing feature gating work
* Continuing exploration of cargo crates
* Reviewed https://github.com/rust-lang/rust/pull/21485

# 2015-01-20

* Responded to cargo versioning issue https://github.com/rust-lang/cargo/issues/1178#issuecomment-70622290
* Requested mailing list be shutdown
* Responded to kill the ml again http://discuss.rust-lang.org/t/is-it-time-to-kill-the-mailing-list/611/44
* Filed bug about windows locking
* Triaged intern resumes
* Added Rust NY to calendar
* Reviewed https://github.com/rust-lang/rust/pull/20919
* Reviewed https://github.com/rust-lang/rust/pull/21065
* Reviewed https://github.com/rust-lang/rust/pull/21138
* Reviewed https://github.com/rust-lang/rust/pull/21274
* Reviewed https://github.com/rust-lang/rust/pull/21413
* Reviewed https://github.com/rust-lang/rust/pull/21429
* Responded to custom LLVM thread http://discuss.rust-lang.org/t/targeted-llvm-for-1-0/1371
* Reviewed https://github.com/rust-lang/rust/pull/21444
* Reviewed https://github.com/brson/multirust/pull/11

# 2015-01-19

* Working on extended errors
* twir
