# financial planning app #

A financial planning & simulation application based on [Electron Boiler Plate](https://github.com/jschr/electron-react-redux-boilerplate) using <a href="http://electron.atom.io/">Electron</a>, <a href="https://facebook.github.io/react/">React</a>, <a href="https://redux.js.org/">Redux</a>, and <a href="https://github.com/reactjs/react-router">React Router</a>.

---

![screenshot_graphs](https://github.com/aero530/fpapp/raw/master/resources/screenshots/graphs.png "Graph")

![screenshot_loan](https://github.com/aero530/fpapp/raw/master/resources/screenshots/loan.png "Loan")

## Features ##

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
* Financial data saved locally as human readable json file

---

## Development Setup ##

This config works when using nodejs installed for windows (not through ubuntu in windows).

### Install / Update Node ###

[https://nodejs.org/en/](https://nodejs.org/en/)

### Install shell launcher ###

Add vs code extension shell launcher.

[https://github.com/Tyriar/vscode-shell-launcher](https://github.com/Tyriar/vscode-shell-launcher)

Use it by ctrl-shift-p 'shell'. Electron apps must be run from cmd.

### Clone the repo via git ###

```cmd
git clone https://github.com/aero530/fpapp.git fpapp
```

And then install dependencies with npm (from the node.js command prompt).

```cmd
> cd fpapp
> npm install
```

Development

```cmd
> npm run develop
```

## Packaging ##

Create a package for macOS, Windows, or Linux using one of the following commands:

```cmd
> npm run pack:mac
> npm run pack:win
> npm run pack:linux
```

## Tests ##

```cmd
> npm run test
```
