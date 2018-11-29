# financial planning app

A financial planning & simulation application based on <a href="https://github.com/chentsulin/electron-react-boilerplate">electron-react-boilerplate</a> using 
<a href="http://electron.atom.io/">Electron</a>, <a href="https://facebook.github.io/react/">React</a>, <a href="https://redux.js.org/">Redux</a>, <a href="https://github.com/reactjs/react-router">React Router</a>, <a href="http://webpack.github.io/docs/">Webpack</a>, and
<a href="https://yarnpkg.com/">yarn</a>.

---

![screenshot_graphs](https://github.com/aero530/fpapp/raw/master/resources/screenshots/graphs.png "Graph")

![screenshot_loan](https://github.com/aero530/fpapp/raw/master/resources/screenshots/loan.png "Loan")

## Features

* Simulate income and expenses through retirement
* Track historic account balances
* Support multiple account types
  * Income
  * Retirement (IRA, Roth IRA, 401K)
  * Social Security
  * College Savings (529)
  * Expenses (such as grocery, car, utilities, insurance, entertainment, rent, etc.)
  * Loans (student, car, etc.)
  * Mortgage
  * Savings
  * Health Savings Account (HSA)
* Make pretty graphs

---

## Development Setup

This config works when using nodejs and yarn installed for windows (not through ubuntu in windows).

### Install / Update Node and yarn:

https://nodejs.org/en/

https://yarnpkg.com/en/

### Install shell launcher:

Add vs code extension shell launcher.

https://github.com/Tyriar/vscode-shell-launcher

Use it by crtl-shft-p 'shell'. Electron apps must be run from cmd.

### Clone the repo via git:

```cmd
git clone --depth=1 https://github.com/chentsulin/electron-react-boilerplate.git your-project-name
```

And then install dependencies with yarn (from the node.js command prompt).

```cmd
$ cd your-project-name
$ yarn
```

## Run

Start the app in the `dev` environment. This starts the renderer process in [**hot-module-replacement**](https://webpack.js.org/guides/hmr-react/) mode and starts a webpack dev server that sends hot updates to the renderer process:

```bash
$ yarn dev
```

Alternatively, you can run the renderer and main processes separately. This way, you can restart one process without waiting for the other. Run these two commands **simultaneously** in different console tabs:

```bash
$ yarn start-renderer-dev
$ yarn start-main-dev
```

If you don't need autofocus when your files was changed, then run `dev` with env `START_MINIMIZED=true`:

```bash
$ START_MINIMIZED=true yarn dev
```

## Packaging

To package apps for the local platform:

```bash
$ yarn package
```

To package apps for all platforms:

First, refer to [Multi Platform Build](https://www.electron.build/multi-platform-build) for dependencies.

Then,

```bash
$ yarn package-all
```

To package apps with options:

```bash
$ yarn package -- --[option]
```

:bulb: You can debug your production build with devtools by simply setting the `DEBUG_PROD` env variable:

```bash
DEBUG_PROD=true yarn package
```

## CSS Modules

This boilerplate is configured to use [css-modules](https://github.com/css-modules/css-modules) out of the box.

All `.css` file extensions will use css-modules unless it has `.global.css`.

If you need global styles, stylesheets with `.global.css` will not go through the
css-modules loader. e.g. `app.global.css`

If you want to import global css libraries (like `bootstrap`), you can just write the following code in `.global.css`:

```css
@import '~bootstrap/dist/css/bootstrap.css';
```

## Dispatching redux actions from main process

See [#118](https://github.com/chentsulin/electron-react-boilerplate/issues/118) and [#108](https://github.com/chentsulin/electron-react-boilerplate/issues/108)

## How to keep your project updated with the boilerplate

If your application is a fork from this repo, you can add this repo to another git remote:

```sh
git remote add upstream https://github.com/chentsulin/electron-react-boilerplate.git
```

Then, use git to merge some latest commits:

```sh
git pull upstream master
```