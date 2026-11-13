# Changelog

## [0.7.0](https://github.com/chainlist/edgestation/compare/v0.6.0...v0.7.0) (2026-11-12)


### Features

* give the settings window its own title bar ([6b735e6](https://github.com/chainlist/edgestation/commit/6b735e67e2ca5af0126076d922f9ccf7f3e4e59c))
* list apps, top programs, Wi-Fi and traffic on Linux ([c6dee91](https://github.com/chainlist/edgestation/commit/c6dee91e1558f02d796c1d85402e2966db545b88))
* pause for full-screen apps and lend the focus on Linux ([06cf587](https://github.com/chainlist/edgestation/commit/06cf587c3ddd8f8407177dace6182549e6a7c939))
* read and change the sound on Linux, through PulseAudio ([046cac8](https://github.com/chainlist/edgestation/commit/046cac8040b4bd5384812f467290cf05b1e7eae6))
* show what plays on Linux, from the MPRIS players ([3a76497](https://github.com/chainlist/edgestation/commit/3a76497b4b72f7b2fbe6053d04e0ca916d54fd98))
* show widget web views at their place on Linux ([d47019e](https://github.com/chainlist/edgestation/commit/d47019e4831da8062d09f3dc28ec94681e668849))
* tint the settings sidebar with the accent color ([04e7c59](https://github.com/chainlist/edgestation/commit/04e7c5975f2a16a73f02d422ee74d34faddd74a9))


### Bug Fixes

* give WebKitGTK a Chrome version and skip view transitions it lacks ([0797656](https://github.com/chainlist/edgestation/commit/07976567b3be72d6eba7855da43f5793728fce66))
* place Settings, identify boxes and the Edge again once they map ([816de25](https://github.com/chainlist/edgestation/commit/816de2529f45484b760c56b528561ae932b36f9d))
* place the Edge, Settings and identify boxes on Linux ([2797cf5](https://github.com/chainlist/edgestation/commit/2797cf5242115de99a6837a4c9ac40c3fe53dbe0))
* put the Edge on its monitor once it maps, and ignore reused ids ([76b23f8](https://github.com/chainlist/edgestation/commit/76b23f80ad4247ac2c5a371247664fd0333ede0b))
* stop Settings &gt; Display from crashing the app on Linux ([238eb69](https://github.com/chainlist/edgestation/commit/238eb69b5973d56c3b042a960ccc553de5748bdb))

## [0.6.0](https://github.com/chainlist/edgestation/compare/v0.5.0...v0.6.0) (2026-11-11)


### Features

* add a widget to a page that has no free space for it ([abab687](https://github.com/chainlist/edgestation/commit/abab6876d63098ec383fc6994b4f2aed90043b85))
* center the Settings content in a column of one width ([67e2873](https://github.com/chainlist/edgestation/commit/67e287368ea51da6379bcf0fceeb53bc773fb190))
* hide the web view buttons while a page is full screen ([aeedb0b](https://github.com/chainlist/edgestation/commit/aeedb0b0d8680de4c2572840c6f70768b9ac98a5))
* install widgets that ask for no permissions without a dialog ([306b364](https://github.com/chainlist/edgestation/commit/306b36467e389608588222b5f1426f5a1bc05ce0))
* redo the Widgets settings page, with a search of installed widgets ([95ece6f](https://github.com/chainlist/edgestation/commit/95ece6f1c67afc3fc88df3c929a7308a54163ceb))
* refresh the marketplace list from Browse ([b72938b](https://github.com/chainlist/edgestation/commit/b72938be97864b8315b3d8812f0c0e9616c93a74))
* show installed widgets as cards, like Browse ([e52655f](https://github.com/chainlist/edgestation/commit/e52655f870915550f42896d58c6462b454059ab3))
* show what widgets ask for, and who publishes them ([5104ae4](https://github.com/chainlist/edgestation/commit/5104ae405195cf4cde37664594eedea032a470e7))
* turn the Edge to the page tab picked in Settings &gt; Pages ([999b0b2](https://github.com/chainlist/edgestation/commit/999b0b257669fda90dbd314e49d26549d0f729c5))

## [0.5.0](https://github.com/chainlist/edgestation/compare/v0.4.1...v0.5.0) (2026-11-11)


### ⚠ BREAKING CHANGES

* "webview": true is replaced by "webview" in "permissions", and a widget without "permissions" can no longer call api.system, api.topPrograms, api.media, api.apps, api.audio, api.fetch or api.webview.

### Features

* added secret settings ([5b27fd4](https://github.com/chainlist/edgestation/commit/5b27fd4dc56e614097720e587008ce3b30ece38c))
* ask widgets to declare permissions, and show them before an install ([35f3e7e](https://github.com/chainlist/edgestation/commit/35f3e7e25f815cbdb45a48646fc38a15344d8c36))
* let widgets show that they load with api.loading ([268d09a](https://github.com/chainlist/edgestation/commit/268d09a6b69094e3a5bda19022470f377cc6ff7c))
* mount a page's widgets the first time it shows ([aedfaaa](https://github.com/chainlist/edgestation/commit/aedfaaa1503c14ead7de9d50de6a02702af36349))
* show web view page loads with the loading dots and bar ([9914158](https://github.com/chainlist/edgestation/commit/9914158ab9fda9ca43336cc6506322b6b3256901))


### Bug Fixes

* count the built-in widgets on the website from the list ([3fd7af1](https://github.com/chainlist/edgestation/commit/3fd7af143c6d49b175e53c120fc1719c4e2ceac2))
* hide URLs and status codes in marketplace errors ([1b35a91](https://github.com/chainlist/edgestation/commit/1b35a9126c9786cb5fd63e6331711c7d9296ba9d))
* let sites in web views use HTML5 drag and drop ([e05d035](https://github.com/chainlist/edgestation/commit/e05d0350c0a1dfcf9ee46a85a744a002e49bd8eb))

## [0.4.1](https://github.com/chainlist/edgestation/compare/v0.4.0...v0.4.1) (2026-11-11)


### Bug Fixes

* keep the Edge from pausing at startup as if it were hidden ([e15b971](https://github.com/chainlist/edgestation/commit/e15b971ca4ae201ec225cc69227bed2a76c30c33))
* let a text field clicked on the Edge take the keyboard ([8368b27](https://github.com/chainlist/edgestation/commit/8368b27d1bde997c998a4d5e8a9b12b084d4ac9b))

## [0.4.0](https://github.com/chainlist/edgestation/compare/v0.3.0...v0.4.0) (2026-11-11)


### Features

* add a weather widget ([34ee40f](https://github.com/chainlist/edgestation/commit/34ee40f3514b13e75992c8275599dd7dcfeb578b))
* add a widget marketplace ([cb73749](https://github.com/chainlist/edgestation/commit/cb73749e1f14c855d82e7c73b4879d94fcdb4599))
* add an audio mixer widget ([e3480ab](https://github.com/chainlist/edgestation/commit/e3480abdcb92ba8c879cccd2aefffc0f18099f50))
* leave the keyboard with the app in front when the Edge is touched ([4ae9109](https://github.com/chainlist/edgestation/commit/4ae9109ef8515c22391a6ce28a996ed887eb1898))
* list built-in widgets under Installed, with a Built-in only filter ([c27a58f](https://github.com/chainlist/edgestation/commit/c27a58fefe036faf7ecf25116acbbffca1da7a1f))
* show the mixer as mute buttons only ([12693be](https://github.com/chainlist/edgestation/commit/12693be4b35eac3ac7cef642f73ea49aafad987d))
* show the weather's details as colored icons ([800de82](https://github.com/chainlist/edgestation/commit/800de82f3a484c7011e0c4e0c6edc17cdb27cde6))
* show the widget marketplace 20 widgets at a time ([b9681bf](https://github.com/chainlist/edgestation/commit/b9681bf7fc0c393aec01b1be7c9dc98efeb981d4))
* tell widgets when the mixer changes, and push them its levels ([8a295c9](https://github.com/chainlist/edgestation/commit/8a295c98d9ef13d542ce06e910994af5ca8c9f08))


### Bug Fixes

* show every app icon when the Edge opens ([10c5ae2](https://github.com/chainlist/edgestation/commit/10c5ae2df9a832cea10a58a8ab9508240e3f5a54))


### Performance Improvements

* make the mixer widget cost nothing while nothing changes or plays ([652964a](https://github.com/chainlist/edgestation/commit/652964a94a099acc09d82536f71a113bd5f85875))

## [0.3.0](https://github.com/chainlist/edgestation/compare/v0.2.0...v0.3.0) (2026-11-10)


### Features

* install updates automatically with the Tauri updater ([cbf1ad1](https://github.com/chainlist/edgestation/commit/cbf1ad1cd9e6b2dd2d2c4b55f389896114e6a966))


### Bug Fixes

* build the built-in widgets into the installers ([2530c89](https://github.com/chainlist/edgestation/commit/2530c89629cc7d4a0945edd1021ab1fea47039cf))

## [0.2.0](https://github.com/chainlist/edgestation/compare/v0.1.1...v0.2.0) (2026-11-10)


### Features

* add a Changelog page to the website ([7ee6989](https://github.com/chainlist/edgestation/commit/7ee6989233b940d769cc6309d84b10d4496c63d5))
* attach the Windows installers to each GitHub release ([10962c6](https://github.com/chainlist/edgestation/commit/10962c6d4f70bcebfbf766fcfe239d377cac381f))
* point the website's Download button at the latest release ([256561e](https://github.com/chainlist/edgestation/commit/256561e0a37d3c0d76018dd7f76f09c63d68c26d))
* warn when a widget's own code calls setTimeout or setInterval ([1d1bac7](https://github.com/chainlist/edgestation/commit/1d1bac72dac23104348e4457851a1dbb515e4087))

## [0.1.1](https://github.com/chainlist/edgestation/compare/v0.1.0...v0.1.1) (2026-11-10)


### Miscellaneous Chores

* release 0.1.1 ([6189ea4](https://github.com/chainlist/edgestation/commit/6189ea46550250de1a83b3e52974f62b14b55555))
