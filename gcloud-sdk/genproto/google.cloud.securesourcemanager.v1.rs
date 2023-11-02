/// A resource that represents a Secure Source Manager instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Optional. A unique identifier for an instance. The name should be of the
    /// format:
    /// `projects/{project_number}/locations/{location_id}/instances/{instance_id}`
    ///
    /// `project_number`: Maps to a unique int64 id assigned to each project.
    ///
    /// `location_id`: Refers to the region where the instance will be deployed.
    /// Since Secure Source Manager is a regional service, it must be one of the
    /// valid GCP regions.
    ///
    /// `instance_id`: User provided name for the instance, must be unique for a
    /// project_number and location_id combination.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Create timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels as key value pairs.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Current state of the instance.
    #[prost(enumeration = "instance::State", tag = "5")]
    pub state: i32,
    /// Output only. An optional field providing information about the current
    /// instance state.
    #[prost(enumeration = "instance::StateNote", tag = "10")]
    pub state_note: i32,
    /// Optional. Immutable. Customer-managed encryption key name, in the format
    /// projects/*/locations/*/keyRings/*/cryptoKeys/*.
    #[prost(string, tag = "11")]
    pub kms_key: ::prost::alloc::string::String,
    /// Output only. A list of hostnames for this instance.
    #[prost(message, optional, tag = "9")]
    pub host_config: ::core::option::Option<instance::HostConfig>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// HostConfig has different instance endpoints.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostConfig {
        /// Output only. HTML hostname.
        #[prost(string, tag = "1")]
        pub html: ::prost::alloc::string::String,
        /// Output only. API hostname. This is the hostname to use for **Host: Data
        /// Plane** endpoints.
        #[prost(string, tag = "2")]
        pub api: ::prost::alloc::string::String,
        /// Output only. Git HTTP hostname.
        #[prost(string, tag = "3")]
        pub git_http: ::prost::alloc::string::String,
        /// Output only. Git SSH hostname.
        #[prost(string, tag = "4")]
        pub git_ssh: ::prost::alloc::string::String,
    }
    /// Secure Source Manager instance state.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Not set. This should only be the case for incoming requests.
        Unspecified = 0,
        /// Instance is being created.
        Creating = 1,
        /// Instance is ready.
        Active = 2,
        /// Instance is being deleted.
        Deleting = 3,
        /// Instance is paused.
        Paused = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Deleting => "DELETING",
                State::Paused => "PAUSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "DELETING" => Some(Self::Deleting),
                "PAUSED" => Some(Self::Paused),
                _ => None,
            }
        }
    }
    /// Provides information about the current instance state.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum StateNote {
        /// STATE_NOTE_UNSPECIFIED as the first value of State.
        Unspecified = 0,
        /// CMEK access is unavailable.
        PausedCmekUnavailable = 1,
        /// INSTANCE_RESUMING indicates that the instance was previously paused
        /// and is under the process of being brought back.
        InstanceResuming = 2,
    }
    impl StateNote {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StateNote::Unspecified => "STATE_NOTE_UNSPECIFIED",
                StateNote::PausedCmekUnavailable => "PAUSED_CMEK_UNAVAILABLE",
                StateNote::InstanceResuming => "INSTANCE_RESUMING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_NOTE_UNSPECIFIED" => Some(Self::Unspecified),
                "PAUSED_CMEK_UNAVAILABLE" => Some(Self::PausedCmekUnavailable),
                "INSTANCE_RESUMING" => Some(Self::InstanceResuming),
                _ => None,
            }
        }
    }
}
/// Metadata of a Secure Source Manager repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// Optional. A unique identifier for a repository. The name should be of the
    /// format:
    /// `projects/{project}/locations/{location_id}/repositories/{repository_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Description of the repository, which cannot exceed 500
    /// characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The name of the instance in which the repository is hosted,
    /// formatted as
    /// `projects/{project_number}/locations/{location_id}/instances/{instance_id}`
    #[prost(string, tag = "3")]
    pub instance: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the repository.
    #[prost(string, tag = "4")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Create timestamp.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update timestamp.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This checksum is computed by the server based on the value of
    /// other fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. URIs for the repository.
    #[prost(message, optional, tag = "9")]
    pub uris: ::core::option::Option<repository::UrIs>,
    /// Input only. Initial configurations for the repository.
    #[prost(message, optional, tag = "10")]
    pub initial_config: ::core::option::Option<repository::InitialConfig>,
}
/// Nested message and enum types in `Repository`.
pub mod repository {
    /// URIs for the repository.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UrIs {
        /// Output only. HTML is the URI for user to view the repository in a
        /// browser.
        #[prost(string, tag = "1")]
        pub html: ::prost::alloc::string::String,
        /// Output only. git_https is the git HTTPS URI for git operations.
        #[prost(string, tag = "2")]
        pub git_https: ::prost::alloc::string::String,
        /// Output only. API is the URI for API access.
        #[prost(string, tag = "3")]
        pub api: ::prost::alloc::string::String,
    }
    /// Repository initialization configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitialConfig {
        /// Default branch name of the repository.
        #[prost(string, tag = "1")]
        pub default_branch: ::prost::alloc::string::String,
        /// List of gitignore template names user can choose from.
        /// Valid values: actionscript, ada, agda, android,
        /// anjuta, ansible, appcelerator-titanium, app-engine, archives,
        /// arch-linux-packages, atmel-studio, autotools, backup, bazaar, bazel,
        /// bitrix, bricx-cc, c, cake-php, calabash, cf-wheels, chef-cookbook,
        /// clojure, cloud9, c-make, code-igniter, code-kit, code-sniffer,
        /// common-lisp, composer, concrete5, coq, cordova, cpp, craft-cms, cuda,
        /// cvs, d, dart, dart-editor, delphi, diff, dm, dreamweaver, dropbox,
        /// drupal, drupal-7, eagle, eclipse, eiffel-studio, elisp, elixir, elm,
        /// emacs, ensime, epi-server, erlang, esp-idf, espresso, exercism,
        /// expression-engine, ext-js, fancy, finale, flex-builder, force-dot-com,
        /// fortran, fuel-php, gcov, git-book, gnome-shell-extension, go, godot, gpg,
        /// gradle, grails, gwt, haskell, hugo, iar-ewarm, idris, igor-pro, images,
        /// infor-cms, java, jboss, jboss-4, jboss-6, jdeveloper, jekyll,
        /// jenkins-home, jenv, jet-brains, jigsaw, joomla, julia, jupyter-notebooks,
        /// kate, kdevelop4, kentico, ki-cad, kohana, kotlin, lab-view, laravel,
        /// lazarus, leiningen, lemon-stand, libre-office, lilypond, linux, lithium,
        /// logtalk, lua, lyx, mac-os, magento, magento-1, magento-2, matlab, maven,
        /// mercurial, mercury, metals, meta-programming-system, meteor,
        /// microsoft-office, model-sim, momentics, mono-develop, nanoc, net-beans,
        /// nikola, nim, ninja, node, notepad-pp, nwjs, objective--c, ocaml, octave,
        /// opa, open-cart, openssl, oracle-forms, otto, packer, patch, perl, perl6,
        /// phalcon, phoenix, pimcore, play-framework, plone, prestashop, processing,
        /// psoc-creator, puppet, pure-script, putty, python, qooxdoo, qt, r, racket,
        /// rails, raku, red, redcar, redis, rhodes-rhomobile, ros, ruby, rust, sam,
        /// sass, sbt, scala, scheme, scons, scrivener, sdcc, seam-gen, sketch-up,
        /// slick-edit, smalltalk, snap, splunk, stata, stella, sublime-text,
        /// sugar-crm, svn, swift, symfony, symphony-cms, synopsys-vcs, tags,
        /// terraform, tex, text-mate, textpattern, think-php, tortoise-git,
        /// turbo-gears-2, typo3, umbraco, unity, unreal-engine, vagrant, vim,
        /// virtual-env, virtuoso, visual-studio, visual-studio-code, vue, vvvv, waf,
        /// web-methods, windows, word-press, xcode, xilinx, xilinx-ise, xojo,
        /// yeoman, yii, zend-framework, zephir.
        #[prost(string, repeated, tag = "2")]
        pub gitignores: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// License template name user can choose from.
        /// Valid values: license-0bsd, license-389-exception, aal, abstyles,
        /// adobe-2006, adobe-glyph, adsl, afl-1-1, afl-1-2, afl-2-0, afl-2-1,
        /// afl-3-0, afmparse, agpl-1-0, agpl-1-0-only, agpl-1-0-or-later,
        /// agpl-3-0-only, agpl-3-0-or-later, aladdin, amdplpa, aml, ampas, antlr-pd,
        /// antlr-pd-fallback, apache-1-0, apache-1-1, apache-2-0, apafml, apl-1-0,
        /// apsl-1-0, apsl-1-1, apsl-1-2, apsl-2-0, artistic-1-0, artistic-1-0-cl8,
        /// artistic-1-0-perl, artistic-2-0, autoconf-exception-2-0,
        /// autoconf-exception-3-0, bahyph, barr, beerware, bison-exception-2-2,
        /// bittorrent-1-0, bittorrent-1-1, blessing, blueoak-1-0-0,
        /// bootloader-exception, borceux, bsd-1-clause, bsd-2-clause,
        /// bsd-2-clause-freebsd, bsd-2-clause-netbsd, bsd-2-clause-patent,
        /// bsd-2-clause-views, bsd-3-clause, bsd-3-clause-attribution,
        /// bsd-3-clause-clear, bsd-3-clause-lbnl, bsd-3-clause-modification,
        /// bsd-3-clause-no-nuclear-license, bsd-3-clause-no-nuclear-license-2014,
        /// bsd-3-clause-no-nuclear-warranty, bsd-3-clause-open-mpi, bsd-4-clause,
        /// bsd-4-clause-shortened, bsd-4-clause-uc, bsd-protection, bsd-source-code,
        /// bsl-1-0, busl-1-1, cal-1-0, cal-1-0-combined-work-exception, caldera,
        /// catosl-1-1, cc0-1-0, cc-by-1-0, cc-by-2-0, cc-by-3-0, cc-by-3-0-at,
        /// cc-by-3-0-us, cc-by-4-0, cc-by-nc-1-0, cc-by-nc-2-0, cc-by-nc-3-0,
        /// cc-by-nc-4-0, cc-by-nc-nd-1-0, cc-by-nc-nd-2-0, cc-by-nc-nd-3-0,
        /// cc-by-nc-nd-3-0-igo, cc-by-nc-nd-4-0, cc-by-nc-sa-1-0, cc-by-nc-sa-2-0,
        /// cc-by-nc-sa-3-0, cc-by-nc-sa-4-0, cc-by-nd-1-0, cc-by-nd-2-0,
        /// cc-by-nd-3-0, cc-by-nd-4-0, cc-by-sa-1-0, cc-by-sa-2-0, cc-by-sa-2-0-uk,
        /// cc-by-sa-2-1-jp, cc-by-sa-3-0, cc-by-sa-3-0-at, cc-by-sa-4-0, cc-pddc,
        /// cddl-1-0, cddl-1-1, cdla-permissive-1-0, cdla-sharing-1-0, cecill-1-0,
        /// cecill-1-1, cecill-2-0, cecill-2-1, cecill-b, cecill-c, cern-ohl-1-1,
        /// cern-ohl-1-2, cern-ohl-p-2-0, cern-ohl-s-2-0, cern-ohl-w-2-0, clartistic,
        /// classpath-exception-2-0, clisp-exception-2-0, cnri-jython, cnri-python,
        /// cnri-python-gpl-compatible, condor-1-1, copyleft-next-0-3-0,
        /// copyleft-next-0-3-1, cpal-1-0, cpl-1-0, cpol-1-02, crossword,
        /// crystal-stacker, cua-opl-1-0, cube, c-uda-1-0, curl, d-fsl-1-0, diffmark,
        /// digirule-foss-exception, doc, dotseqn, drl-1-0, dsdp, dvipdfm, ecl-1-0,
        /// ecl-2-0, ecos-exception-2-0, efl-1-0, efl-2-0, egenix, entessa, epics,
        /// epl-1-0, epl-2-0, erlpl-1-1, etalab-2-0, eu-datagrid, eupl-1-0, eupl-1-1,
        /// eupl-1-2, eurosym, fair, fawkes-runtime-exception, fltk-exception,
        /// font-exception-2-0, frameworx-1-0, freebsd-doc, freeimage,
        /// freertos-exception-2-0, fsfap, fsful, fsfullr, ftl, gcc-exception-2-0,
        /// gcc-exception-3-1, gd, gfdl-1-1-invariants-only,
        /// gfdl-1-1-invariants-or-later, gfdl-1-1-no-invariants-only,
        /// gfdl-1-1-no-invariants-or-later, gfdl-1-1-only, gfdl-1-1-or-later,
        /// gfdl-1-2-invariants-only, gfdl-1-2-invariants-or-later,
        /// gfdl-1-2-no-invariants-only, gfdl-1-2-no-invariants-or-later,
        /// gfdl-1-2-only, gfdl-1-2-or-later, gfdl-1-3-invariants-only,
        /// gfdl-1-3-invariants-or-later, gfdl-1-3-no-invariants-only,
        /// gfdl-1-3-no-invariants-or-later, gfdl-1-3-only, gfdl-1-3-or-later,
        /// giftware, gl2ps, glide, glulxe, glwtpl, gnu-javamail-exception, gnuplot,
        /// gpl-1-0-only, gpl-1-0-or-later, gpl-2-0-only, gpl-2-0-or-later,
        /// gpl-3-0-linking-exception, gpl-3-0-linking-source-exception,
        /// gpl-3-0-only, gpl-3-0-or-later, gpl-cc-1-0, gsoap-1-3b, haskell-report,
        /// hippocratic-2-1, hpnd, hpnd-sell-variant, htmltidy,
        /// i2p-gpl-java-exception, ibm-pibs, icu, ijg, image-magick, imatix, imlib2,
        /// info-zip, intel, intel-acpi, interbase-1-0, ipa, ipl-1-0, isc,
        /// jasper-2-0, jpnic, json, lal-1-2, lal-1-3, latex2e, leptonica,
        /// lgpl-2-0-only, lgpl-2-0-or-later, lgpl-2-1-only, lgpl-2-1-or-later,
        /// lgpl-3-0-linking-exception, lgpl-3-0-only, lgpl-3-0-or-later, lgpllr,
        /// libpng, libpng-2-0, libselinux-1-0, libtiff, libtool-exception,
        /// liliq-p-1-1, liliq-r-1-1, liliq-rplus-1-1, linux-openib,
        /// linux-syscall-note, llvm-exception, lpl-1-0, lpl-1-02, lppl-1-0,
        /// lppl-1-1, lppl-1-2, lppl-1-3a, lppl-1-3c, lzma-exception, make-index,
        /// mif-exception, miros, mit, mit-0, mit-advertising, mit-cmu, mit-enna,
        /// mit-feh, mit-modern-variant, mitnfa, mit-open-group, motosoto, mpich2,
        /// mpl-1-0, mpl-1-1, mpl-2-0, mpl-2-0-no-copyleft-exception, ms-pl, ms-rl,
        /// mtll, mulanpsl-1-0, mulanpsl-2-0, multics, mup, naist-2003, nasa-1-3,
        /// naumen, nbpl-1-0, ncgl-uk-2-0, ncsa, netcdf, net-snmp, newsletr, ngpl,
        /// nist-pd, nist-pd-fallback, nlod-1-0, nlpl, nokia, nokia-qt-exception-1-1,
        /// nosl, noweb, npl-1-0, npl-1-1, nposl-3-0, nrl, ntp, ntp-0,
        /// ocaml-lgpl-linking-exception, occt-exception-1-0, occt-pl, oclc-2-0,
        /// odbl-1-0, odc-by-1-0, ofl-1-0, ofl-1-0-no-rfn, ofl-1-0-rfn, ofl-1-1,
        /// ofl-1-1-no-rfn, ofl-1-1-rfn, ogc-1-0, ogdl-taiwan-1-0, ogl-canada-2-0,
        /// ogl-uk-1-0, ogl-uk-2-0, ogl-uk-3-0, ogtsl, oldap-1-1, oldap-1-2,
        /// oldap-1-3, oldap-1-4, oldap-2-0, oldap-2-0-1, oldap-2-1, oldap-2-2,
        /// oldap-2-2-1, oldap-2-2-2, oldap-2-3, oldap-2-4, oldap-2-7, oml,
        /// openjdk-assembly-exception-1-0, openssl, openvpn-openssl-exception,
        /// opl-1-0, oset-pl-2-1, osl-1-0, osl-1-1, osl-2-0, osl-2-1, osl-3-0,
        /// o-uda-1-0, parity-6-0-0, parity-7-0-0, pddl-1-0, php-3-0, php-3-01,
        /// plexus, polyform-noncommercial-1-0-0, polyform-small-business-1-0-0,
        /// postgresql, psf-2-0, psfrag, ps-or-pdf-font-exception-20170817, psutils,
        /// python-2-0, qhull, qpl-1-0, qt-gpl-exception-1-0, qt-lgpl-exception-1-1,
        /// qwt-exception-1-0, rdisc, rhecos-1-1, rpl-1-1, rpsl-1-0, rsa-md, rscpl,
        /// ruby, saxpath, sax-pd, scea, sendmail, sendmail-8-23, sgi-b-1-0,
        /// sgi-b-1-1, sgi-b-2-0, shl-0-51, shl-2-0, shl-2-1, simpl-2-0, sissl,
        /// sissl-1-2, sleepycat, smlnj, smppl, snia, spencer-86, spencer-94,
        /// spencer-99, spl-1-0, ssh-openssh, ssh-short, sspl-1-0, sugarcrm-1-1-3,
        /// swift-exception, swl, tapr-ohl-1-0, tcl, tcp-wrappers, tmate, torque-1-1,
        /// tosl, tu-berlin-1-0, tu-berlin-2-0, u-boot-exception-2-0, ucl-1-0,
        /// unicode-dfs-2015, unicode-dfs-2016, unicode-tou,
        /// universal-foss-exception-1-0, unlicense, upl-1-0, vim, vostrom, vsl-1-0,
        /// w3c, w3c-19980720, w3c-20150513, watcom-1-0, wsuipa, wtfpl,
        /// wxwindows-exception-3-1, x11, xerox, xfree86-1-1, xinetd, xnet, xpp,
        /// xskat, ypl-1-0, ypl-1-1, zed, zend-2-0, zimbra-1-3, zimbra-1-4, zlib,
        /// zlib-acknowledgement, zpl-1-1, zpl-2-0, zpl-2-1.
        #[prost(string, tag = "3")]
        pub license: ::prost::alloc::string::String,
        /// README template name.
        /// Valid template name(s) are: default.
        #[prost(string, tag = "4")]
        pub readme: ::prost::alloc::string::String,
    }
}
/// ListInstancesRequest is the request to list instances.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Parent value for ListInstancesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter for filtering results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetInstanceRequest is the request for getting an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateInstanceRequest is the request for creating an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the instance to be created.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// DeleteInstanceRequest is the request for deleting an instance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Name of the resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// ListRepositoriesRequest is request to list repositories.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. Parent value for ListRepositoriesRequest.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Requested page size. Server may return fewer items than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// The list of repositories.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// GetRepositoryRequest is the request for getting a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. Name of the repository to retrieve.
    /// The format is
    /// `projects/{project_number}/locations/{location_id}/repositories/{repository_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateRepositoryRequest is the request for creating a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepositoryRequest {
    /// Required. The project in which to create the repository. Values are of the
    /// form `projects/{project_number}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource being created.
    #[prost(message, optional, tag = "2")]
    pub repository: ::core::option::Option<Repository>,
    /// Required. The ID to use for the repository, which will become the final
    /// component of the repository's resource name. This value should be 4-63
    /// characters, and valid characters are /[a-z][0-9]-/.
    #[prost(string, tag = "3")]
    pub repository_id: ::prost::alloc::string::String,
}
/// DeleteRepositoryRequest is the request to delete a repository.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepositoryRequest {
    /// Required. Name of the repository to delete.
    /// The format is
    /// projects/{project_number}/locations/{location_id}/repositories/{repository_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set to true, and the repository is not found, the request will
    /// succeed but no action will be taken on the server.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
}
/// Generated client implementations.
pub mod secure_source_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Secure Source Manager API
    ///
    /// Access Secure Source Manager instances, resources, and repositories.
    ///
    /// This API is split across two servers: the Control Plane and the Data Plane.
    ///
    /// Data Plane endpoints are hosted directly by your Secure Source Manager
    /// instance, so you must connect to your instance's API hostname to access
    /// them. The API hostname looks like the following:
    ///
    ///    https://[instance-id]-[project-number]-api.[location].sourcemanager.dev
    ///
    /// For example,
    ///
    ///    https://my-instance-702770452863-api.us-central1.sourcemanager.dev
    ///
    /// Data Plane endpoints are denoted with **Host: Data Plane**.
    ///
    /// All other endpoints are found in the normal Cloud API location, namely,
    /// `securcesourcemanager.googleapis.com`.
    #[derive(Debug, Clone)]
    pub struct SecureSourceManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SecureSourceManagerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SecureSourceManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SecureSourceManagerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SecureSourceManagerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Lists Instances in a given project and location.
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListInstancesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/ListInstances",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "ListInstances",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets details of a single instance.
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> std::result::Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/GetInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "GetInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new instance in a given project and location.
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/CreateInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "CreateInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a single instance.
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/DeleteInstance",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "DeleteInstance",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists Repositories in a given project and location.
        ///
        /// **Host: Data Plane**
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRepositoriesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/ListRepositories",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "ListRepositories",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets metadata of a repository.
        ///
        /// **Host: Data Plane**
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> std::result::Result<tonic::Response<super::Repository>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/GetRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "GetRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new repository in a given project and location.
        ///
        /// **Host: Data Plane**
        pub async fn create_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepositoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/CreateRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "CreateRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes a Repository.
        ///
        /// **Host: Data Plane**
        pub async fn delete_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepositoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/DeleteRepository",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "DeleteRepository",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get IAM policy for a repository.
        pub async fn get_iam_policy_repo(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/GetIamPolicyRepo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "GetIamPolicyRepo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Set IAM policy on a repository.
        pub async fn set_iam_policy_repo(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/SetIamPolicyRepo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "SetIamPolicyRepo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Test IAM permissions on a repository.
        /// IAM permission checks are not required on this method.
        pub async fn test_iam_permissions_repo(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.securesourcemanager.v1.SecureSourceManager/TestIamPermissionsRepo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.securesourcemanager.v1.SecureSourceManager",
                        "TestIamPermissionsRepo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
