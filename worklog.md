---
layout: basic
---

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
