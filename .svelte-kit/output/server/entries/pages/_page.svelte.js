import { c as create_ssr_component, b as add_attribute, a as subscribe, v as validate_component, e as escape, d as each, f as createEventDispatcher, o as onDestroy, m as missing_component } from "../../chunks/ssr.js";
import "@tauri-apps/api/event";
import "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { w as writable } from "../../chunks/index.js";
import { v4 } from "uuid";
import "d3";
let defaultCollege = {
  type: "college",
  name: "",
  table: {},
  contributions: {},
  earnings: {},
  withdrawals: {},
  startIn: 0,
  endIn: 0,
  startOut: 0,
  endOut: 0,
  contributionValue: 0,
  contributionType: "fixed",
  yearlyReturn: "",
  withdrawalType: "fixed",
  withdrawalValue: 0,
  taxStatus: "contribute_taxed_earnings_untaxed_when_used",
  notes: ""
};
let defaultExpense = {
  type: "expense",
  name: "",
  table: {},
  startOut: 0,
  endOut: 0,
  expenseType: "fixed",
  expenseValue: 0,
  isHealthcare: false,
  hsaLink: "",
  notes: ""
};
let defaultHsa = {
  type: "hsa",
  name: "",
  table: {},
  startIn: 0,
  endIn: 0,
  startOut: 0,
  endOut: 0,
  contributionValue: 0,
  contributionType: "fixed",
  employerContribution: 0,
  yearlyReturn: "",
  withdrawalType: "fixed",
  withdrawalValue: 0,
  taxStatus: "contribute_taxed_earnings_untaxed_when_used",
  notes: ""
};
let defaultIncome = {
  type: "income",
  name: "",
  table: {},
  base: 0,
  startIn: 0,
  endIn: 0,
  raise: "",
  notes: ""
};
let defaultLoan = {
  type: "loan",
  name: "",
  table: {},
  startOut: 0,
  endOut: 0,
  paymentType: "fixed",
  paymentValue: 0,
  rate: "",
  notes: ""
};
let defaultMortgage = {
  type: "mortgage",
  name: "",
  table: {},
  startOut: 0,
  endOut: 0,
  paymentType: "fixed",
  paymentValue: 0,
  rate: "",
  compoundTime: 0,
  mortgageInsurance: 0,
  ltvLimit: 0,
  escrowValue: 0,
  homeValue: 0,
  notes: ""
};
let defaultRetirement = {
  type: "retirement",
  name: "",
  table: {},
  contributions: {},
  earnings: {},
  withdrawals: {},
  employerContributions: {},
  startIn: 0,
  endIn: 0,
  startOut: 0,
  endOut: 0,
  contributionValue: 0,
  contributionType: "fixed",
  yearlyReturn: "",
  withdrawalType: "fixed",
  withdrawalValue: 0,
  taxStatus: "contribute_taxed_earnings_untaxed_when_used",
  incomeLink: "",
  matching: {
    amount: 0,
    limit: 0
  },
  notes: ""
};
let defaultSavings = {
  type: "savings",
  name: "",
  table: {},
  contributions: {},
  earnings: {},
  withdrawals: {},
  startIn: 0,
  endIn: 0,
  startOut: 0,
  endOut: 0,
  contributionValue: 0,
  contributionType: "fixed",
  yearlyReturn: "",
  withdrawalType: "fixed",
  withdrawalValue: 0,
  taxStatus: "contribute_taxed_earnings_untaxed_when_used",
  notes: ""
};
let defaultSsa = {
  type: "ssa",
  name: "",
  base: 0,
  startIn: 0,
  endIn: 0,
  notes: ""
};
function run_analysis(inputs) {
  invoke("run_analysis", {
    input: { ...inputs }
  }).then((results) => {
    plot_data.set(results[0]);
    summary_data.set(results[1]);
    console.log(results);
  });
}
let defaultInput = {
  accounts: {},
  settings: {
    ageRetire: 65,
    ageDie: 90,
    yearBorn: 1950,
    yearStart: 2e3,
    inflationBase: 10,
    taxIncome: 10,
    taxCapitalGains: 10,
    retirementCostOfLiving: 100,
    ssa: {
      breakpoints: { low: 0, high: 0 },
      taxableIncomePercentage: { low: 0, high: 0 }
    }
  }
};
function createInput() {
  const { subscribe: subscribe2, set, update } = writable(defaultInput);
  return {
    subscribe: subscribe2,
    set: (inputs) => {
      set(inputs);
      run_analysis(inputs);
    },
    removeTableRow: (id, table, year) => {
      update((current) => {
        delete current.accounts[id][table][year];
        run_analysis(current);
        return current;
      });
    },
    addTableRow: (id, table, year, value) => {
      update((current) => {
        current.accounts[id][table][year] = value;
        run_analysis(current);
        return current;
      });
    },
    removeAccount: (id) => {
      update((current) => {
        delete current.accounts[id];
        run_analysis(current);
        return current;
      });
    },
    addAccount: (type) => {
      let id = v4();
      update((current) => {
        switch (type) {
          case 0:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultCollege));
            break;
          case 1:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultExpense));
            break;
          case 2:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultHsa));
            break;
          case 3:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultIncome));
            break;
          case 4:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultLoan));
            break;
          case 5:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultMortgage));
            break;
          case 6:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultRetirement));
            break;
          case 7:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultSavings));
            break;
          case 8:
            current.accounts[id] = JSON.parse(JSON.stringify(defaultSsa));
            break;
        }
        run_analysis(current);
        return current;
      });
    },
    reset: () => set(defaultInput)
  };
}
const form_inputs = createInput();
function createPath() {
  const { subscribe: subscribe2, set } = writable("");
  return {
    subscribe: subscribe2,
    set: (input) => set(input),
    reset: () => set("")
  };
}
const path = createPath();
function createPlotData() {
  const { subscribe: subscribe2, set, update } = writable({});
  return {
    subscribe: subscribe2,
    set: (input) => set(input),
    reset: () => set({})
  };
}
const plot_data = createPlotData();
let defaultSummary = {
  col: [],
  expense: [],
  healthcare_expense: [],
  hsa: [],
  income: [],
  income_during_retirement: [],
  income_taxable: [],
  net: [],
  saving: [],
  tax_burden: []
};
function createSummaryData() {
  const { subscribe: subscribe2, set, update } = writable(defaultSummary);
  return {
    subscribe: subscribe2,
    set: (input) => set(input),
    reset: () => set(defaultSummary)
  };
}
const summary_data = createSummaryData();
const dark = writable(false);
var NumberFormat = /* @__PURE__ */ ((NumberFormat2) => {
  NumberFormat2["Decimal"] = "~r";
  NumberFormat2["Scientific"] = "~e";
  NumberFormat2["DateQuarter"] = "Q%q/%y";
  NumberFormat2["SI"] = "~s";
  return NumberFormat2;
})(NumberFormat || {});
const Line = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { data } = $$props;
  let { title = "" } = $$props;
  let { xLabel = "" } = $$props;
  let { yLabel = "" } = $$props;
  let { displayLegend = false } = $$props;
  let { format = {
    box: { width: 150, height: 100 },
    title: {
      fontSize: 5,
      paddingBottom: 8,
      translate: { x: 0, y: 0 }
    },
    axis: {
      tick: { fontSize: 3, padding: 2, size: 2 },
      label: { fontSize: 4 },
      strokeWidth: 0.3,
      format: {
        x: NumberFormat.Decimal,
        y: NumberFormat.Decimal
      }
    },
    tooltip: {
      circleRadius: 1,
      circleStrokeWidth: 0.4,
      borderWidth: 1,
      borderRadius: 2,
      padding: 5,
      format: {
        x: NumberFormat.Decimal,
        y: NumberFormat.Decimal
      }
    },
    legend: { fontSize: 2, translate: { x: 2, y: 2 } },
    lineStrokeWidth: 0.6,
    dataPointSize: 0,
    fontAR: 0.5
  } } = $$props;
  let { domain = {
    x: { min: null, max: null },
    y: { min: null, max: null }
  } } = $$props;
  let wr;
  let el;
  if ($$props.data === void 0 && $$bindings.data && data !== void 0) $$bindings.data(data);
  if ($$props.title === void 0 && $$bindings.title && title !== void 0) $$bindings.title(title);
  if ($$props.xLabel === void 0 && $$bindings.xLabel && xLabel !== void 0) $$bindings.xLabel(xLabel);
  if ($$props.yLabel === void 0 && $$bindings.yLabel && yLabel !== void 0) $$bindings.yLabel(yLabel);
  if ($$props.displayLegend === void 0 && $$bindings.displayLegend && displayLegend !== void 0) $$bindings.displayLegend(displayLegend);
  if ($$props.format === void 0 && $$bindings.format && format !== void 0) $$bindings.format(format);
  if ($$props.domain === void 0 && $$bindings.domain && domain !== void 0) $$bindings.domain(domain);
  return `<div class="overflow-visible relative"${add_attribute("this", wr, 0)}><svg${add_attribute("this", el, 0)}></svg></div>`;
});
function toPoints(data) {
  let arr = [];
  Object.entries(data).forEach((entry) => {
    const [key, value] = entry;
    arr.push({ x: parseInt(key), y: value });
  });
  return arr;
}
const Dashboard = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $summary_data, $$unsubscribe_summary_data;
  $$unsubscribe_summary_data = subscribe(summary_data, (value) => $summary_data = value);
  let { zeroBase = true } = $$props;
  let domain = {
    x: { min: null, max: null },
    y: { min: zeroBase ? 0 : null, max: null }
  };
  if ($$props.zeroBase === void 0 && $$bindings.zeroBase && zeroBase !== void 0) $$bindings.zeroBase(zeroBase);
  $$unsubscribe_summary_data();
  return `${Object.keys($summary_data.col).length == 0 ? `<div class="grid grid-cols-3 gap-4 place-content-center h-48" data-svelte-h="svelte-rkh935"><div></div> <div><h2 class="text-lg text-center">to begin load data from file or configure settings &amp; add accounts</h2></div> <div></div></div>` : `<div class="grid grid-cols-1 gap-1 mx-32">${validate_component(Line, "Line").$$render(
    $$result,
    {
      data: [
        {
          label: "Cost of Living",
          data: toPoints($summary_data.col)
        },
        {
          label: "Expenses",
          data: toPoints($summary_data.expense)
        },
        {
          label: "Income",
          data: toPoints($summary_data.income)
        },
        {
          label: "Net",
          data: toPoints($summary_data.net)
        }
      ],
      title: "Overview",
      xLabel: "Year",
      yLabel: "$",
      displayLegend: true
    },
    {},
    {}
  )} ${validate_component(Line, "Line").$$render(
    $$result,
    {
      data: toPoints($summary_data.saving),
      title: "Savings",
      xLabel: "Year",
      yLabel: "$"
    },
    {},
    {}
  )} ${validate_component(Line, "Line").$$render(
    $$result,
    {
      data: toPoints($summary_data.col),
      title: "Cost of Living",
      xLabel: "Year",
      yLabel: "$",
      domain
    },
    {},
    {}
  )}</div>`}`;
});
const css$3 = {
  code: ".tooltip.svelte-1rsx3ir.svelte-1rsx3ir{visibility:hidden;position:absolute\n}.has-tooltip.svelte-1rsx3ir:hover .tooltip.svelte-1rsx3ir{visibility:visible;z-index:50\n}",
  map: '{"version":3,"file":"QuestionField.svelte","sources":["QuestionField.svelte"],"sourcesContent":["<script lang=\\"ts\\">import { v4 as uuidv4 } from \\"uuid\\";\\nexport let label = \\"\\";\\nlet size = 24;\\nlet padding = 2;\\nlet id = uuidv4();\\n<\/script>\\n\\n<div class=\\"flex items-center\\">\\n\\t<div class=\\"mr-3 text-md\\">\\n\\t\\t<label for=\\"{id}\\" class=\\"text-dark dark:text-light\\">{label}</label>\\n\\t</div>\\n\\t<div class=\\"flex grow items-center\\">\\n\\t\\t<slot id=\\"{id}\\"/>\\n\\t\\t<div class=\\"w-6 h-6 ml-2 has-tooltip\\">\\n\\t\\t\\t<svg xmlns=\\"http://www.w3.org/2000/svg\\" fill=\\"currentColor\\" style=\\"padding:{padding}px; height:{size}px; width:{size}px; margin:0px;\\" viewBox=\\"0 0 32 32\\">\\n\\t\\t\\t\\t<defs>\\n\\t\\t\\t\\t<style>.cls-1{fill:none;}</style>\\n\\t\\t\\t\\t</defs>\\n\\t\\t\\t\\t<path d=\\"M16,2A14,14,0,1,0,30,16,14,14,0,0,0,16,2Zm0,26A12,12,0,1,1,28,16,12,12,0,0,1,16,28Z\\"/>\\n\\t\\t\\t\\t<circle cx=\\"16\\" cy=\\"23.5\\" r=\\"1.5\\"/>\\n\\t\\t\\t\\t<path d=\\"M17,8H15.5A4.49,4.49,0,0,0,11,12.5V13h2v-.5A2.5,2.5,0,0,1,15.5,10H17a2.5,2.5,0,0,1,0,5H15v4.5h2V17a4.5,4.5,0,0,0,0-9Z\\"/>\\n\\t\\t\\t\\t<rect class=\\"cls-1\\" width=\\"32\\" height=\\"32\\"/>\\n\\t\\t\\t</svg >\\n\\t\\t\\t<div class=\\"tooltip px-1 py-1 -mt-10 ml-6 text-center border rounded-sm bg-background-400 dark:bg-darkbackground-300 border-dark\\"><slot name=\\"helper\\"/></div>\\n\\t\\t</div>\\n\\t</div>\\n</div>\\n\\n<style lang=\\"postcss\\">\\n\\t.tooltip {\\n\\n    visibility: hidden;\\n\\n    position: absolute\\n}\\n\\n\\t.has-tooltip:hover .tooltip {\\n\\n    visibility: visible;\\n\\n    z-index: 50\\n}\\n</style>"],"names":[],"mappings":"AA6BC,sCAAS,CAEN,UAAU,CAAE,MAAM,CAElB,QAAQ,CAAE,QAAQ;AACtB,CAEC,2BAAY,MAAM,CAAC,uBAAS,CAEzB,UAAU,CAAE,OAAO,CAEnB,OAAO,CAAE,EAAE;AACf"}'
};
let size = 24;
let padding = 2;
const QuestionField = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label = "" } = $$props;
  let id = v4();
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  $$result.css.add(css$3);
  return `<div class="flex items-center"><div class="mr-3 text-md"><label${add_attribute("for", id, 0)} class="text-dark dark:text-light">${escape(label)}</label></div> <div class="flex grow items-center">${slots.default ? slots.default({ id }) : ``} <div class="w-6 h-6 ml-2 has-tooltip svelte-1rsx3ir"><svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" style="${"padding:" + escape(padding, true) + "px; height:" + escape(size, true) + "px; width:" + escape(size, true) + "px; margin:0px;"}" viewBox="0 0 32 32"><defs><style>.cls-1{fill:none;}</style></defs><path d="M16,2A14,14,0,1,0,30,16,14,14,0,0,0,16,2Zm0,26A12,12,0,1,1,28,16,12,12,0,0,1,16,28Z"></path><circle cx="16" cy="23.5" r="1.5"></circle><path d="M17,8H15.5A4.49,4.49,0,0,0,11,12.5V13h2v-.5A2.5,2.5,0,0,1,15.5,10H17a2.5,2.5,0,0,1,0,5H15v4.5h2V17a4.5,4.5,0,0,0,0-9Z"></path><rect class="cls-1" width="32" height="32"></rect></svg> <div class="tooltip px-1 py-1 -mt-10 ml-6 text-center border rounded-sm bg-background-400 dark:bg-darkbackground-300 border-dark svelte-1rsx3ir">${slots.helper ? slots.helper({}) : ``}</div></div></div> </div>`;
});
const NumberInput = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let { questionText = "" } = $$props;
  let { step = 1 } = $$props;
  let inputValue = 1;
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  if ($$props.step === void 0 && $$bindings.step && step !== void 0) $$bindings.step(step);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText)}</div>`;
    },
    default: () => {
      return `<input type="number" class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"${add_attribute("value", inputValue, 0)}>`;
    }
  })}`;
});
let defaultHelp = "Years can use variables (yearStart, yearEnd, yearRetire, yearDie), numbers, or equations (such as yearStart+4 or yearEnd-10)";
function parseValue$1(input) {
  if (typeof input === "object" && input !== null) {
    return input.base + (input.delta > 0 ? "+" + input.delta.toString() : input.delta.toString());
  } else {
    return input;
  }
}
const YearInput = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let { questionText = "" } = $$props;
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText + defaultHelp)}</div>`;
    },
    default: () => {
      return `<input type="text"${add_attribute("value", parseValue$1(value), 0)} class="${[
        "p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400",
        ""
      ].join(" ").trim()}">`;
    }
  })}`;
});
const Settings = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `<div class="container mx-auto columns-1">${$form_inputs.settings.hasOwnProperty("ageDie") ? `<h1 class="mt-4 mb-2 text-xl font-bold" data-svelte-h="svelte-c8xmn">Settings</h1> <div class="grid grid-cols-2 gap-4 border rounded p-4 bg-background-600 dark:bg-darkbackground-600">${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "ageDie",
        step: 1,
        questionText: "Age when you might die (for financial purposes as least)",
        value: $form_inputs.settings.ageDie
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ageDie = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "ageRetire",
        step: 1,
        questionText: "The age at which you plan to retire",
        value: $form_inputs.settings.ageRetire
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ageRetire = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(YearInput, "YearInput").$$render(
      $$result,
      {
        label: "yearBorn",
        questionText: "Year in which you were born",
        value: $form_inputs.settings.yearBorn
      },
      {
        value: ($$value) => {
          $form_inputs.settings.yearBorn = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(YearInput, "YearInput").$$render(
      $$result,
      {
        label: "yearStart",
        questionText: "Year to start the simulation",
        value: $form_inputs.settings.yearStart
      },
      {
        value: ($$value) => {
          $form_inputs.settings.yearStart = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "inflationBase",
        questionText: "Base rate of inflation (percent)",
        value: $form_inputs.settings.inflationBase
      },
      {
        value: ($$value) => {
          $form_inputs.settings.inflationBase = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "retirementCostOfLiving",
        questionText: "Fraction of current spending when retired (such as in retirement you will spend 80% of what you spend now)",
        value: $form_inputs.settings.retirementCostOfLiving
      },
      {
        value: ($$value) => {
          $form_inputs.settings.retirementCostOfLiving = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "taxCapitalGains",
        questionText: "Tax rate for capital gains",
        value: $form_inputs.settings.taxCapitalGains
      },
      {
        value: ($$value) => {
          $form_inputs.settings.taxCapitalGains = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "taxIncome",
        questionText: "Tax rate for your income bracket",
        value: $form_inputs.settings.taxIncome
      },
      {
        value: ($$value) => {
          $form_inputs.settings.taxIncome = $$value;
          $$settled = false;
        }
      },
      {}
    )}</div> <h1 class="mt-6 mb-2 text-xl font-bold" data-svelte-h="svelte-1y0xex4">Social Security Settings</h1> <div class="border rounded p-4 bg-background-600 dark:bg-darkbackground-600"><div class="grid grid-cols-2 gap-4">${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "Lower Income Bound",
        questionText: "Lower income bound for social security",
        value: $form_inputs.settings.ssa.breakpoints.low
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ssa.breakpoints.low = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "Upper Income Bound",
        questionText: "Upper income bound for social security",
        value: $form_inputs.settings.ssa.breakpoints.high
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ssa.breakpoints.high = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "Lower taxableIncomePercentage",
        questionText: "Lower taxableIncomePercentage for social security",
        value: $form_inputs.settings.ssa.taxableIncomePercentage.low
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ssa.taxableIncomePercentage.low = $$value;
          $$settled = false;
        }
      },
      {}
    )} ${validate_component(NumberInput, "NumberInput").$$render(
      $$result,
      {
        label: "Upper taxableIncomePercentage",
        questionText: "Upper taxableIncomePercentage for social security",
        value: $form_inputs.settings.ssa.taxableIncomePercentage.high
      },
      {
        value: ($$value) => {
          $form_inputs.settings.ssa.taxableIncomePercentage.high = $$value;
          $$settled = false;
        }
      },
      {}
    )}</div></div>` : `<div class="grid grid-cols-2 gap-4 border rounded p-4 bg-background-600 dark:bg-darkbackground-600" data-svelte-h="svelte-cohoep">Please load data</div>`}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Line_1 = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let domain;
  let $plot_data, $$unsubscribe_plot_data;
  $$unsubscribe_plot_data = subscribe(plot_data, (value) => $plot_data = value);
  let { id } = $$props;
  let { title } = $$props;
  let { xLabel } = $$props;
  let { yLabel } = $$props;
  let { displayLegend = false } = $$props;
  let data;
  if ($$props.id === void 0 && $$bindings.id && id !== void 0) $$bindings.id(id);
  if ($$props.title === void 0 && $$bindings.title && title !== void 0) $$bindings.title(title);
  if ($$props.xLabel === void 0 && $$bindings.xLabel && xLabel !== void 0) $$bindings.xLabel(xLabel);
  if ($$props.yLabel === void 0 && $$bindings.yLabel && yLabel !== void 0) $$bindings.yLabel(yLabel);
  if ($$props.displayLegend === void 0 && $$bindings.displayLegend && displayLegend !== void 0) $$bindings.displayLegend(displayLegend);
  data = $plot_data.hasOwnProperty(id) ? $plot_data[id] : [];
  domain = $plot_data.hasOwnProperty(id) ? {
    x: { min: null, max: null },
    y: { min: 0, max: null }
  } : {
    x: { min: null, max: null },
    y: { min: 0, max: null }
  };
  $$unsubscribe_plot_data();
  return `${$plot_data.hasOwnProperty(id) ? `${validate_component(Line, "Line").$$render(
    $$result,
    {
      data,
      title,
      xLabel,
      yLabel,
      domain,
      displayLegend
    },
    {},
    {}
  )}` : ``}`;
});
const Contribution = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let options = [
    {
      value: "fixed",
      label: "fixed",
      description: "fixed dollar amount"
    },
    {
      value: "percent_of_income",
      label: "percent of income",
      description: "percent of cost of current living"
    },
    {
      value: "fixed_with_inflation",
      label: "fixed with inflation",
      description: "fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)"
    }
  ];
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper"><b data-svelte-h="svelte-1c7izwr">Select how money is put into the account.</b> <ul>${each(options, (option) => {
        return `<li>${escape(option.label)} : ${escape(option.description)}</li>`;
      })}</ul></div>`;
    },
    default: () => {
      return `<select class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(options, (option) => {
        return `<option${add_attribute("value", option.value, 0)}>${escape(option.label)}</option>`;
      })}</select>`;
    }
  })}`;
});
const Withdrawal = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let options = [
    {
      value: "fixed",
      label: "Fixed",
      description: "Take out a fixed dollar amount"
    },
    {
      value: "fixed_with_inflation",
      label: "Fixed with inflation",
      description: "Take out a fixed dollar amount with inflation compensation"
    },
    {
      value: "end_at_zero",
      label: "End at zero",
      description: "take money out in equal amounts each year such that the balance at endOut is zero"
    },
    {
      value: "col_frac_of_savings",
      label: "Cost of living",
      description: "Take out the current cost of living * (this accounts value / total savings)"
    },
    {
      value: "percent_of_income",
      label: "Percent of income",
      description: "Take out a percent of income in each year"
    }
  ];
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper"><b data-svelte-h="svelte-1aaxdfs">Select how money is taken out of the account.</b> <ul>${each(options, (option) => {
        return `<li>${escape(option.label)} : ${escape(option.description)}</li>`;
      })}</ul></div>`;
    },
    default: () => {
      return `<select class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(options, (option) => {
        return `<option${add_attribute("value", option.value, 0)}>${escape(option.label)}</option>`;
      })}</select>`;
    }
  })}`;
});
const TaxStatus = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let items = [
    {
      value: "contribute_taxed_earnings_untaxed_when_used",
      text: "Earnings are taxed deferred",
      description: "payed with taxed income, earnings are tax deferred, withdrawals are not taxed"
    },
    {
      value: "contribute_taxed_earnings_taxed",
      text: "Earings taxes as capital gains",
      description: "payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed (tax free as long as used for intended purpose)"
    },
    {
      value: "not_impliemented",
      text: "not implemented",
      description: "NOT IMPLEMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed"
    },
    {
      value: "contribute_pretax_taxed_when_used",
      text: "Taxed as income when used",
      description: "payed pretax and taxed in year of use as income"
    },
    {
      value: "contribute_pretax_untaxed_when_used",
      text: "Not taxed",
      description: "payed pretax and not taxed as income (use with HSA)"
    }
  ];
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper"><b data-svelte-h="svelte-h8ghh0">How taxes impact this account.</b> <ul>${each(items, (item) => {
        return `<li>${escape(item.text)} : ${escape(item.description)}</li>`;
      })}</ul></div>`;
    },
    default: () => {
      return `<select class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(items, (item) => {
        return `<option${add_attribute("value", item.value, 0)}>${escape(item.text)}</option>`;
      })}</select>`;
    }
  })}`;
});
const TextInput = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value = "" } = $$props;
  let { questionText = "" } = $$props;
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText)}</div>`;
    },
    default: () => {
      return `<input type="text" class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"${add_attribute("value", value, 0)}>`;
    }
  })}`;
});
const TextAreaInput = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value = "" } = $$props;
  let { questionText = "" } = $$props;
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText)}</div>`;
    },
    default: () => {
      return `<textarea textarea class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${escape(value || "")}</textarea>`;
    }
  })}`;
});
function parseValue(input) {
  if (typeof input === "object" && input !== null) {
    return input.base + (input.delta > 0 ? "+" + input.delta.toString() : input.delta.toString());
  } else {
    return input;
  }
}
const PercentInput = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let { questionText = "Percentage can be a number (such at 15) or inflationBase." } = $$props;
  let invalid = false;
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText)}</div>`;
    },
    default: () => {
      return `<input${add_attribute("value", parseValue(value), 0)}${add_attribute("invalid", invalid, 0)} class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400 border">`;
    }
  })}`;
});
const AddAlt = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { size: size2 = 24 } = $$props;
  let { padding: padding2 = 2 } = $$props;
  if ($$props.size === void 0 && $$bindings.size && size2 !== void 0) $$bindings.size(size2);
  if ($$props.padding === void 0 && $$bindings.padding && padding2 !== void 0) $$bindings.padding(padding2);
  return `<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" style="${"padding:" + escape(padding2, true) + "px; height:" + escape(size2, true) + "px; width:" + escape(size2, true) + "px; margin:0px;"}" viewBox="0 0 32 32"><style type="text/css">.st0{fill:none;}
    </style><path d="M16,4c6.6,0,12,5.4,12,12s-5.4,12-12,12S4,22.6,4,16S9.4,4,16,4 M16,2C8.3,2,2,8.3,2,16s6.3,14,14,14s14-6.3,14-14
        S23.7,2,16,2z"></path><polygon points="24,15 17,15 17,8 15,8 15,15 8,15 8,17 15,17 15,24 17,24 17,17 24,17 "></polygon><rect id="_Transparent_Rectangle_" class="st0" width="32" height="32"></rect></svg>`;
});
const SubtractAlt = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { size: size2 = 24 } = $$props;
  let { padding: padding2 = 2 } = $$props;
  if ($$props.size === void 0 && $$bindings.size && size2 !== void 0) $$bindings.size(size2);
  if ($$props.padding === void 0 && $$bindings.padding && padding2 !== void 0) $$bindings.padding(padding2);
  return `<svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" style="${"padding:" + escape(padding2, true) + "px; height:" + escape(size2, true) + "px; width:" + escape(size2, true) + "px; margin:0px;"}" viewBox="0 0 32 32"><style type="text/css">.st0{fill:none;}
    </style><path d="M16,4c6.6,0,12,5.4,12,12s-5.4,12-12,12S4,22.6,4,16S9.4,4,16,4 M16,2C8.3,2,2,8.3,2,16s6.3,14,14,14s14-6.3,14-14
        S23.7,2,16,2z"></path><rect x="8" y="15" width="16" height="2"></rect><rect id="_Transparent_Rectangle_" class="st0" width="32" height="32"></rect></svg>`;
});
const Table = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { data } = $$props;
  let inputYear = "";
  let inputValue = "";
  createEventDispatcher();
  let id = v4();
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.data === void 0 && $$bindings.data && data !== void 0) $$bindings.data(data);
  return `<div class="grid grid-col-1 w-fit gap-2 border rounded p-1"><div class="text-md"><label${add_attribute("for", id, 0)} class="font-medium text-gray-700">${escape(label)}</label></div> <div class="flex grow items-center"><table class="table-auto"><thead data-svelte-h="svelte-1qxmtfj"><tr><th>Year</th> <th>Amount</th> <th></th></tr></thead> <tbody>${data !== null ? `${each(Object.keys(data).sort(), (thisYear) => {
    return `<tr><td class="pl-1">${escape(thisYear)}</td> <td class="pl-1">${escape(data[thisYear])}</td> <td><div>${validate_component(SubtractAlt, "SubtractAlt").$$render($$result, {}, {}, {})} </div></td> </tr>`;
  })}` : ``} <tr><td><input type="number" class="p-0 m-0 pl-1 w-16 text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"${add_attribute("value", inputYear, 0)}></td> <td><input type="number" class="p-0 m-0 pl-1 w-32 text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400"${add_attribute("value", inputValue, 0)}></td> <td><div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></td></tr></tbody></table></div></div>`;
});
const Modal = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { title = "" } = $$props;
  createEventDispatcher();
  if ($$props.title === void 0 && $$bindings.title && title !== void 0) $$bindings.title(title);
  return `<div class="fixed inset-0 w-screen h-screen bg-black opacity-50"></div> <div class="fixed inset-0 m-auto bg-background-500 w-fit h-fit rounded">${title !== "" ? `<div class="bg-secondary-500 rounded-t w-full p-2"><h1 class="text-light font-bold text-xl">${escape(title)}</h1></div>` : ``} <div class="p-6">${slots.default ? slots.default({}) : ``}</div></div>`;
});
const DeleteModal = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let { open = false } = $$props;
  let { id = "" } = $$props;
  if ($$props.open === void 0 && $$bindings.open && open !== void 0) $$bindings.open(open);
  if ($$props.id === void 0 && $$bindings.id && id !== void 0) $$bindings.id(id);
  $$unsubscribe_form_inputs();
  return `${open ? `${validate_component(Modal, "Modal").$$render(
    $$result,
    {
      title: `Delete Account ${$form_inputs.accounts[id].name}?`
    },
    {},
    {
      default: () => {
        return `<button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1a9t5dt">Yes</button> <button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1awpogf">No</button>`;
      }
    }
  )}` : ``}`;
});
const AccountCard = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<div class="grid grid-rows-1 gap-2 border rounded p-4 bg-background-600 dark:bg-darkbackground-600"><div class="flex flex-wrap gap-2"><div class="flex-initial w-fit">${slots.inputs ? slots.inputs({}) : ``}</div> <div class="flex-auto w-[32rem]">${slots.chart ? slots.chart({}) : ``}</div></div> <div class="flex flex-wrap gap-4">${slots.balance ? slots.balance({}) : ``} ${slots.emp_contributions ? slots.emp_contributions({}) : ``} ${slots.contributions ? slots.contributions({}) : ``} ${slots.earnings ? slots.earnings({}) : ``} ${slots.withdrawals ? slots.withdrawals({}) : ``}</div></div>`;
});
const College = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-1ag35yv">College Savings</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "college" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        withdrawals: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "withdrawals",
              label: "Withdrawals",
              data: $form_inputs.accounts[id].withdrawals
            },
            {},
            {}
          )}`;
        },
        earnings: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "earnings",
              label: "Earnings",
              data: $form_inputs.accounts[id].earnings
            },
            {},
            {}
          )}`;
        },
        contributions: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "contributions",
              label: "Contributions",
              data: $form_inputs.accounts[id].contributions
            },
            {},
            {}
          )}`;
        },
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 dark:bg-primary-300 dark:hover:bg-primary-200 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2" data-svelte-h="svelte-1virufe">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "When money will start going into this account...",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "When money will stop going into this account...",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Contribution Value",
              step: 1,
              questionText: "Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]",
              value: $form_inputs.accounts[id].contributionValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(Contribution, "Contribution").$$render(
            $$result,
            {
              label: "Contribution Type",
              value: $form_inputs.accounts[id].contributionType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Withdrawal Value",
              step: 1,
              questionText: "How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].withdrawalValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(Withdrawal, "Withdrawal").$$render(
            $$result,
            {
              label: "Withdrawal Type",
              value: $form_inputs.accounts[id].withdrawalType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Yearly Return",
              questionText: "Percent interest earned each year",
              value: $form_inputs.accounts[id].yearlyReturn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].yearlyReturn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TaxStatus, "TaxStatus").$$render(
            $$result,
            {
              label: "Tax Status",
              value: $form_inputs.accounts[id].taxStatus
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].taxStatus = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Expense = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let options = [
    {
      value: "fixed",
      label: "fixed",
      description: "fixed dollar amount"
    },
    {
      value: "fixed_with_inflation",
      label: "fixed with inflation",
      description: "fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)"
    }
  ];
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper"><b data-svelte-h="svelte-1c7izwr">Select how money is put into the account.</b> <ul>${each(options, (option) => {
        return `<li>${escape(option.label)} : ${escape(option.description)}</li>`;
      })}</ul></div>`;
    },
    default: () => {
      return `<select class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(options, (option) => {
        return `<option${add_attribute("value", option.value, 0)}>${escape(option.label)}</option>`;
      })}</select>`;
    }
  })}`;
});
const css$2 = {
  code: "input.svelte-1dqlfgo:checked~.dot.svelte-1dqlfgo{transform:translateX(200%);--tw-bg-opacity:1;background-color:rgb(15 98 254 / var(--tw-bg-opacity))}",
  map: '{"version":3,"file":"AccountLink.svelte","sources":["AccountLink.svelte"],"sourcesContent":["<script lang=\\"ts\\">import { v4 as uuidv4 } from \\"uuid\\";\\nimport QuestionField from \\"./QuestionField.svelte\\";\\nexport let label = \\"\\";\\nexport let account = null;\\nexport let questionText = \\"\\";\\nexport let accounts = {};\\nexport let accountTypeFilter = \\"\\";\\nexport let isOn = false;\\n$: options = Object.keys(accounts).filter((id2) => accountTypeFilter ? accounts[id2].type == accountTypeFilter : true).map((id2) => {\\n  return { value: id2, label: accounts[id2].name };\\n});\\nlet account_id = account;\\nlet id = uuidv4();\\n$: account = isOn ? account_id : null;\\n<\/script>\\n\\n<QuestionField {label}>\\n    <div class=\\"grow flex items-center w-fit gap-4\\">\\n        <label for={id} class=\\"flex items-center cursor-pointer\\">\\n            <!-- toggle -->\\n            <div class=\\"relative\\">\\n                <!-- input -->\\n                <input id={id} type=\\"checkbox\\" class=\\"sr-only\\" bind:checked={isOn}/>\\n                <!-- line -->\\n                <div class=\\"w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner\\"></div>\\n                <!-- dot -->\\n                <div class=\\"dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition\\"></div>\\n            </div>\\n        </label>\\n    \\n        {#if isOn}\\n            <select bind:value={account_id} class=\\"p-0 m-0 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400\\">\\n                {#each options as option (option.label)}\\n                    <option value={option.value}>{option.label}</option>\\n                {/each}\\n            </select>\\n        {/if}\\n    </div>\\n    <div slot=\\"helper\\">{questionText}</div>\\n</QuestionField>\\n\\n<style lang=\\"postcss\\">\\n    input:checked ~ .dot {\\n        transform: translateX(200%);\\n        --tw-bg-opacity: 1;\\n        background-color: rgb(15 98 254 / var(--tw-bg-opacity));\\n    }\\n</style>"],"names":[],"mappings":"AA0CI,oBAAK,QAAQ,CAAG,mBAAK,CACjB,SAAS,CAAE,WAAW,IAAI,CAAC,CAC3B,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D"}'
};
const AccountLink = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let options;
  let { label = "" } = $$props;
  let { account = null } = $$props;
  let { questionText = "" } = $$props;
  let { accounts = {} } = $$props;
  let { accountTypeFilter = "" } = $$props;
  let { isOn = false } = $$props;
  let account_id = account;
  let id = v4();
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.account === void 0 && $$bindings.account && account !== void 0) $$bindings.account(account);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  if ($$props.accounts === void 0 && $$bindings.accounts && accounts !== void 0) $$bindings.accounts(accounts);
  if ($$props.accountTypeFilter === void 0 && $$bindings.accountTypeFilter && accountTypeFilter !== void 0) $$bindings.accountTypeFilter(accountTypeFilter);
  if ($$props.isOn === void 0 && $$bindings.isOn && isOn !== void 0) $$bindings.isOn(isOn);
  $$result.css.add(css$2);
  options = Object.keys(accounts).filter((id2) => accountTypeFilter ? accounts[id2].type == accountTypeFilter : true).map((id2) => {
    return { value: id2, label: accounts[id2].name };
  });
  account = isOn ? account_id : null;
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper">${escape(questionText)}</div>`;
    },
    default: () => {
      return `<div class="grow flex items-center w-fit gap-4"><label${add_attribute("for", id, 0)} class="flex items-center cursor-pointer"> <div class="relative"> <input${add_attribute("id", id, 0)} type="checkbox" class="sr-only svelte-1dqlfgo"${add_attribute("checked", isOn, 1)}>  <div class="w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner"></div>  <div class="dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition svelte-1dqlfgo"></div></div></label> ${isOn ? `<select class="p-0 m-0 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(options, (option) => {
        return `<option${add_attribute("value", option.value, 0)}>${escape(option.label)}</option>`;
      })}</select>` : ``}</div>`;
    }
  })}`;
});
const Expenses = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-176f4u2">Expenses</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "expense" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount"
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Expense Value",
              step: 1,
              questionText: "Yearly cost of the expense [in today's dollars]",
              value: $form_inputs.accounts[id].expenseValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].expenseValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Expense, "Expense").$$render(
            $$result,
            {
              label: "Expense Type",
              value: $form_inputs.accounts[id].expenseType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].expenseType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(AccountLink, "AccountLink").$$render(
            $$result,
            {
              label: "Healthcare cost?",
              accounts: $form_inputs.accounts,
              accountTypeFilter: "hsa",
              questionText: "Is this a healthcare cost that should be paid for out of HSA",
              isOn: $form_inputs.accounts[id].isHealthcare,
              account: $form_inputs.accounts[id].hsaLink
            },
            {
              isOn: ($$value) => {
                $form_inputs.accounts[id].isHealthcare = $$value;
                $$settled = false;
              },
              account: ($$value) => {
                $form_inputs.accounts[id].hsaLink = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Hsa = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-1v5idnt">HSA</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "hsa" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "When money will start going into this account...",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "When money will stop going into this account...",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Contribution Value",
              step: 1,
              questionText: "Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]",
              value: $form_inputs.accounts[id].contributionValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(Contribution, "Contribution").$$render(
            $$result,
            {
              label: "Contribution Type",
              value: $form_inputs.accounts[id].contributionType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Withdrawal Value",
              step: 1,
              questionText: "How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].withdrawalValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(Withdrawal, "Withdrawal").$$render(
            $$result,
            {
              label: "Withdrawal Type",
              value: $form_inputs.accounts[id].withdrawalType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Yearly Return",
              questionText: "Percent interest earned each year",
              value: $form_inputs.accounts[id].yearlyReturn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].yearlyReturn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TaxStatus, "TaxStatus").$$render(
            $$result,
            {
              label: "Tax Status",
              value: $form_inputs.accounts[id].taxStatus
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].taxStatus = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Employer Contribution",
              step: 1,
              questionText: "Employer contributions to this account as a dollar amount [in today's dollars]",
              value: $form_inputs.accounts[id].employerContribution
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].employerContribution = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Income = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-1b7la9g">Income</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "income" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount"
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "Calendar year when money starts being earned by this account",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "Calendar year when money stops being earned by this account",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Base Pay",
              step: 1,
              questionText: "Base pay (with bonuses) [in today's dollars]",
              value: $form_inputs.accounts[id].base
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].base = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Raise",
              questionText: "Yearly increase in income as a percent",
              value: $form_inputs.accounts[id].raise
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].raise = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Payment = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label } = $$props;
  let { value } = $$props;
  let options = [
    {
      value: "fixed",
      label: "fixed",
      description: "fixed dollar amount"
    },
    {
      value: "fixed_with_inflation",
      label: "fixed with inflation",
      description: "fixed dollar amount compensated for inflation from year start (ie dollar amount is in current dollars)"
    }
  ];
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.value === void 0 && $$bindings.value && value !== void 0) $$bindings.value(value);
  return `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
    helper: () => {
      return `<div slot="helper"><b data-svelte-h="svelte-20gukt">Select how payment value should be interpreted.</b> <ul>${each(options, (option) => {
        return `<li>${escape(option.label)} : ${escape(option.description)}</li>`;
      })}</ul></div>`;
    },
    default: () => {
      return `<select class="p-0 m-0 pl-1 grow text-dark dark:text-light bg-background-400 dark:bg-darkbackground-400">${each(options, (option) => {
        return `<option${add_attribute("value", option.value, 0)}>${escape(option.label)}</option>`;
      })}</select>`;
    }
  })}`;
});
const Loan = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-s4efep">Loan</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "loan" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Payment, "Payment").$$render(
            $$result,
            {
              label: "Payment Type",
              value: $form_inputs.accounts[id].paymentType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].paymentType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Payment Value",
              step: 1,
              questionText: "How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].paymentValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].paymentValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Interest Rate",
              questionText: "Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.",
              value: $form_inputs.accounts[id].rate
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].rate = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Compound Freq",
              step: 1,
              questionText: "Number of times per year that interest is compounded. (1=yearly, 12=monthly)",
              value: $form_inputs.accounts[id].compoundTime
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].compoundTime = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Mortgage = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-cy5c19">Mortgage</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "mortgage" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Payment, "Payment").$$render(
            $$result,
            {
              label: "Payment Type",
              value: $form_inputs.accounts[id].paymentType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].paymentType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Payment Value",
              step: 1,
              questionText: "How much money should be payed each year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].paymentValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].paymentValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Interest Rate",
              questionText: "Interest rate on borrowed money. This is an APR this is then compounded based on the compound time setting.  Used for LOAN and MORTGAGE account types.",
              value: $form_inputs.accounts[id].rate
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].rate = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Compound Freq",
              step: 1,
              questionText: "Number of times per year that interest is compounded. (1=yearly, 12=monthly)",
              value: $form_inputs.accounts[id].compoundTime
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].compoundTime = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Mortgage Insurance",
              step: 1,
              questionText: "Mortgage insurance payment expressed as a yearly fixed number [in today's dollars]",
              value: $form_inputs.accounts[id].mortgageInsurance
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].mortgageInsurance = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Loan to Value",
              step: 1,
              questionText: "Loan to Value amount when mortgage insurance is no longer pulled from payment.  Since monthly payment does not change over time, after the insurance is done there is more money going to the principal each payment",
              value: $form_inputs.accounts[id].ltvLimit
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].ltvLimit = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Escrow",
              step: 1,
              questionText: "Amount of money going into escrow every year to pay for property tax.  This number is currently assumed to be constant (ie property taxes do not increase) [in today's dollars]",
              value: $form_inputs.accounts[id].escrowValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].escrowValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Home Value",
              step: 1,
              questionText: "Current value of the home.  This is used to compute loan to value [in today's dollars]",
              value: $form_inputs.accounts[id].homeValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].homeValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const css$1 = {
  code: "input.svelte-1dqlfgo:checked~.dot.svelte-1dqlfgo{transform:translateX(200%);--tw-bg-opacity:1;background-color:rgb(15 98 254 / var(--tw-bg-opacity))}",
  map: '{"version":3,"file":"EmployerMatch.svelte","sources":["EmployerMatch.svelte"],"sourcesContent":["<script lang=\\"ts\\">import { v4 as uuidv4 } from \\"uuid\\";\\nimport QuestionField from \\"./QuestionField.svelte\\";\\nimport PercentInput from \\"./PercentInput.svelte\\";\\nexport let label = \\"\\";\\nexport let questionText = \\"\\";\\nexport let matching = { \\"amount\\": 0, \\"limit\\": 0 };\\nlet amount = matching && matching.hasOwnProperty(\\"amount\\") ? matching.amount : 0;\\nlet limit = matching && matching.hasOwnProperty(\\"limit\\") ? matching.limit : 0;\\nlet enabled = amount > 0 || limit > 0;\\nlet id = uuidv4();\\n$: matching = enabled ? {\\n  \\"amount\\": amount,\\n  \\"limit\\": limit\\n} : null;\\n<\/script>\\n\\n<QuestionField {label}>\\n    <div class=\\"grow flex items-center w-fit gap-4\\">\\n\\n        <label for={id} class=\\"flex items-center cursor-pointer\\">\\n            <!-- toggle -->\\n            <div class=\\"relative\\">\\n                <!-- input -->\\n                <input id={id} type=\\"checkbox\\" class=\\"sr-only\\" bind:checked={enabled}/>\\n                <!-- line -->\\n                <div class=\\"w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner\\"></div>\\n                <!-- dot -->\\n                <div class=\\"dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition\\"></div>\\n            </div>\\n        </label>\\n\\n        {#if enabled}\\n            <div class=\\"grid grid-cols-10 gap-2\\">\\n                <div class=\\"col-span-10\\">\\n                    <PercentInput\\n                    label=\\"Employer Match\\"\\n                    bind:value={amount}\\n                    questionText=\\"% of what you put in that the employer matches\\"\\n                    />\\n                </div>\\n                <div class=\\"col-span-10\\">\\n                <PercentInput\\n                label=\\"Match Limit\\"\\n                bind:value={limit}\\n                questionText=\\"% of what you put in when the employer stops matching\\"\\n                />\\n                </div>\\n            </div>\\n        {/if}\\n    </div>\\n    <div slot=\\"helper\\">{questionText}</div>\\n</QuestionField>\\n\\n<style lang=postcss>\\n    input:checked ~ .dot {\\n        transform: translateX(200%);\\n        --tw-bg-opacity: 1;\\n        background-color: rgb(15 98 254 / var(--tw-bg-opacity));\\n    }\\n</style>"],"names":[],"mappings":"AAsDI,oBAAK,QAAQ,CAAG,mBAAK,CACjB,SAAS,CAAE,WAAW,IAAI,CAAC,CAC3B,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D"}'
};
const EmployerMatch = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { label = "" } = $$props;
  let { questionText = "" } = $$props;
  let { matching = { "amount": 0, "limit": 0 } } = $$props;
  let amount = matching && matching.hasOwnProperty("amount") ? matching.amount : 0;
  let limit = matching && matching.hasOwnProperty("limit") ? matching.limit : 0;
  let enabled = amount > 0 || limit > 0;
  let id = v4();
  if ($$props.label === void 0 && $$bindings.label && label !== void 0) $$bindings.label(label);
  if ($$props.questionText === void 0 && $$bindings.questionText && questionText !== void 0) $$bindings.questionText(questionText);
  if ($$props.matching === void 0 && $$bindings.matching && matching !== void 0) $$bindings.matching(matching);
  $$result.css.add(css$1);
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    matching = enabled ? { amount, limit } : null;
    $$rendered = `${validate_component(QuestionField, "QuestionField").$$render($$result, { label }, {}, {
      helper: () => {
        return `<div slot="helper">${escape(questionText)}</div>`;
      },
      default: () => {
        return `<div class="grow flex items-center w-fit gap-4"><label${add_attribute("for", id, 0)} class="flex items-center cursor-pointer"> <div class="relative"> <input${add_attribute("id", id, 0)} type="checkbox" class="sr-only svelte-1dqlfgo"${add_attribute("checked", enabled, 1)}>  <div class="w-10 h-2 bg-background-600 dark:bg-darkbackground-600 rounded-full shadow-inner"></div>  <div class="dot absolute w-4 h-4 bg-white dark:bg-darkbackground-300 rounded-full shadow -left-1 -top-1 transition svelte-1dqlfgo"></div></div></label> ${enabled ? `<div class="grid grid-cols-10 gap-2"><div class="col-span-10">${validate_component(PercentInput, "PercentInput").$$render(
          $$result,
          {
            label: "Employer Match",
            questionText: "% of what you put in that the employer matches",
            value: amount
          },
          {
            value: ($$value) => {
              amount = $$value;
              $$settled = false;
            }
          },
          {}
        )}</div> <div class="col-span-10">${validate_component(PercentInput, "PercentInput").$$render(
          $$result,
          {
            label: "Match Limit",
            questionText: "% of what you put in when the employer stops matching",
            value: limit
          },
          {
            value: ($$value) => {
              limit = $$value;
              $$settled = false;
            }
          },
          {}
        )}</div></div>` : ``}</div>`;
      }
    })}`;
  } while (!$$settled);
  return $$rendered;
});
const Retirement = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-1lfi7zw">Retirement</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "retirement" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        withdrawals: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "withdrawals",
              label: "Withdrawals",
              data: $form_inputs.accounts[id].withdrawals
            },
            {},
            {}
          )}`;
        },
        earnings: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "earnings",
              label: "Earnings",
              data: $form_inputs.accounts[id].earnings
            },
            {},
            {}
          )}`;
        },
        emp_contributions: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "emp_contributions",
              label: "Employer Contributions",
              data: $form_inputs.accounts[id].employerContributions
            },
            {},
            {}
          )}`;
        },
        contributions: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "contributions",
              label: "Contributions",
              data: $form_inputs.accounts[id].contributions
            },
            {},
            {}
          )}`;
        },
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "When money will start going into this account...",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "When money will stop going into this account...",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Contribution Value",
              step: 1,
              questionText: "Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. If an income link is configured the percentage will come from the value for that account otherwise it will be from the total income from the year. [in today's dollars]",
              value: $form_inputs.accounts[id].contributionValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Contribution, "Contribution").$$render(
            $$result,
            {
              label: "Contribution Type",
              value: $form_inputs.accounts[id].contributionType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Withdrawal Value",
              step: 1,
              questionText: "How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].withdrawalValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Withdrawal, "Withdrawal").$$render(
            $$result,
            {
              label: "Withdrawal Type",
              value: $form_inputs.accounts[id].withdrawalType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Yearly Return",
              questionText: "Percent interest earned each year",
              value: $form_inputs.accounts[id].yearlyReturn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].yearlyReturn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TaxStatus, "TaxStatus").$$render(
            $$result,
            {
              label: "Tax Status",
              value: $form_inputs.accounts[id].taxStatus
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].taxStatus = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(AccountLink, "AccountLink").$$render(
            $$result,
            {
              label: "Income link?",
              accounts: $form_inputs.accounts,
              accountTypeFilter: "income",
              questionText: "Is this account linked to an income account",
              account: $form_inputs.accounts[id].incomeLink
            },
            {
              account: ($$value) => {
                $form_inputs.accounts[id].incomeLink = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(EmployerMatch, "EmployerMatch").$$render(
            $$result,
            {
              label: "Employer Match",
              questionText: "Does employer match your contributions made into this account?",
              matching: $form_inputs.accounts[id].matching
            },
            {
              matching: ($$value) => {
                $form_inputs.accounts[id].matching = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Savings = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-15vs4zc">Savings</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "savings" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        withdrawals: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "withdrawals",
              label: "Withdrawals",
              data: $form_inputs.accounts[id].withdrawals
            },
            {},
            {}
          )}`;
        },
        earnings: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "earnings",
              label: "Earnings",
              data: $form_inputs.accounts[id].earnings
            },
            {},
            {}
          )}`;
        },
        contributions: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "contributions",
              label: "Contributions",
              data: $form_inputs.accounts[id].contributions
            },
            {},
            {}
          )}`;
        },
        balance: () => {
          return `${validate_component(Table, "Table").$$render(
            $$result,
            {
              slot: "balance",
              label: "Balance",
              data: $form_inputs.accounts[id].table
            },
            {},
            {}
          )}`;
        },
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount",
              displayLegend: true
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "When money will start going into this account...",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "When money will stop going into this account...",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start Out",
              questionText: "When money will start coming out of this account...",
              value: $form_inputs.accounts[id].startOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End Out",
              questionText: "When money will stop coming out of this account...",
              value: $form_inputs.accounts[id].endOut
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endOut = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Contribution Value",
              step: 1,
              questionText: "Amount put into this account every year.  Numbers less than 100 are assumed to be a percentage. [in today's dollars]",
              value: $form_inputs.accounts[id].contributionValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Contribution, "Contribution").$$render(
            $$result,
            {
              label: "Contribution Type",
              value: $form_inputs.accounts[id].contributionType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].contributionType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Withdrawal Value",
              step: 1,
              questionText: "How much money should be take out per year (either as a percentage or a fixed dollar amount) [in today's dollars]",
              value: $form_inputs.accounts[id].withdrawalValue
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalValue = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(Withdrawal, "Withdrawal").$$render(
            $$result,
            {
              label: "Withdrawal Type",
              value: $form_inputs.accounts[id].withdrawalType
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].withdrawalType = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(PercentInput, "PercentInput").$$render(
            $$result,
            {
              label: "Yearly Return",
              questionText: "Percent interest earned each year",
              value: $form_inputs.accounts[id].yearlyReturn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].yearlyReturn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(TaxStatus, "TaxStatus").$$render(
            $$result,
            {
              label: "Tax Status",
              value: $form_inputs.accounts[id].taxStatus
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].taxStatus = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const Ssa = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $form_inputs, $$unsubscribe_form_inputs;
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => $form_inputs = value);
  let deleteModal = { open: false, id: "" };
  let $$settled;
  let $$rendered;
  let previous_head = $$result.head;
  do {
    $$settled = true;
    $$result.head = previous_head;
    $$rendered = `${validate_component(DeleteModal, "DeleteModal").$$render(
      $$result,
      {
        id: deleteModal.id,
        open: deleteModal.open
      },
      {},
      {}
    )} <div class="flex items-center"><div class="text-lg pr-2" data-svelte-h="svelte-y6iv7y">SSA</div> <div>${validate_component(AddAlt, "AddAlt").$$render($$result, {}, {}, {})}</div></div> <div class="grid grid-cols-1 gap-4">${each(Object.keys($form_inputs.accounts), (id) => {
      return `${$form_inputs.accounts[id].type == "ssa" ? `${validate_component(AccountCard, "AccountCard").$$render($$result, {}, {}, {
        chart: () => {
          return `${validate_component(Line_1, "Line").$$render(
            $$result,
            {
              slot: "chart",
              id,
              title: $form_inputs.accounts[id].name,
              xLabel: "Year",
              yLabel: "Amount"
            },
            {},
            {}
          )}`;
        },
        inputs: () => {
          return `<div slot="inputs" class="grid grid-cols-10 gap-2"><div class="col-span-7">${validate_component(TextInput, "TextInput").$$render(
            $$result,
            {
              label: "Account name",
              questionText: "Human friendly name for the account",
              value: $form_inputs.accounts[id].name
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].name = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-3 flex grow items-center"><button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-2 py-1 text-center mx-2 dark:bg-primary-300 dark:hover:bg-primary-200" data-svelte-h="svelte-1bbrws6">Delete Account
				</button></div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "Start In",
              questionText: "When money will start going into this account...",
              value: $form_inputs.accounts[id].startIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].startIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(YearInput, "YearInput").$$render(
            $$result,
            {
              label: "End In",
              questionText: "When money will stop going into this account...",
              value: $form_inputs.accounts[id].endIn
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].endIn = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-5">${validate_component(NumberInput, "NumberInput").$$render(
            $$result,
            {
              label: "Base",
              step: 1,
              questionText: "Base income from social security",
              value: $form_inputs.accounts[id].base
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].base = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> <div class="col-span-10">${validate_component(TextAreaInput, "TextAreaInput").$$render(
            $$result,
            {
              label: "Notes",
              questionText: "General information to store with this account",
              value: $form_inputs.accounts[id].notes
            },
            {
              value: ($$value) => {
                $form_inputs.accounts[id].notes = $$value;
                $$settled = false;
              }
            },
            {}
          )}</div> </div>`;
        }
      })}` : ``}`;
    })}</div>`;
  } while (!$$settled);
  $$unsubscribe_form_inputs();
  return $$rendered;
});
const css = {
  code: `.svelte-7w3m9l,.svelte-7w3m9l::before,.svelte-7w3m9l::after{box-sizing:border-box;border-width:0;border-style:solid;border-color:currentColor}.svelte-7w3m9l::before,.svelte-7w3m9l::after{--tw-content:''}:host{line-height:1.5;-webkit-text-size-adjust:100%;-moz-tab-size:4;-o-tab-size:4;tab-size:4;font-family:ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";font-feature-settings:normal;font-variation-settings:normal;-webkit-tap-highlight-color:transparent}hr.svelte-7w3m9l{height:0;color:inherit;border-top-width:1px}button.svelte-7w3m9l{font-family:inherit;font-feature-settings:inherit;font-variation-settings:inherit;font-size:100%;font-weight:inherit;line-height:inherit;letter-spacing:inherit;color:inherit;margin:0;padding:0}button.svelte-7w3m9l{text-transform:none}button.svelte-7w3m9l{-webkit-appearance:button;background-color:transparent;background-image:none}.svelte-7w3m9l:-moz-focusring{outline:auto}.svelte-7w3m9l:-moz-ui-invalid{box-shadow:none}.svelte-7w3m9l::-webkit-inner-spin-button,.svelte-7w3m9l::-webkit-outer-spin-button{height:auto}.svelte-7w3m9l::-webkit-search-decoration{-webkit-appearance:none}.svelte-7w3m9l::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}hr.svelte-7w3m9l{margin:0}ul.svelte-7w3m9l{list-style:none;margin:0;padding:0}button.svelte-7w3m9l{cursor:pointer}.svelte-7w3m9l:disabled{cursor:default}.svelte-7w3m9l,.svelte-7w3m9l::before,.svelte-7w3m9l::after{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x:  ;--tw-pan-y:  ;--tw-pinch-zoom:  ;--tw-scroll-snap-strictness:proximity;--tw-gradient-from-position:  ;--tw-gradient-via-position:  ;--tw-gradient-to-position:  ;--tw-ordinal:  ;--tw-slashed-zero:  ;--tw-numeric-figure:  ;--tw-numeric-spacing:  ;--tw-numeric-fraction:  ;--tw-ring-inset:  ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur:  ;--tw-brightness:  ;--tw-contrast:  ;--tw-grayscale:  ;--tw-hue-rotate:  ;--tw-invert:  ;--tw-saturate:  ;--tw-sepia:  ;--tw-drop-shadow:  ;--tw-backdrop-blur:  ;--tw-backdrop-brightness:  ;--tw-backdrop-contrast:  ;--tw-backdrop-grayscale:  ;--tw-backdrop-hue-rotate:  ;--tw-backdrop-invert:  ;--tw-backdrop-opacity:  ;--tw-backdrop-saturate:  ;--tw-backdrop-sepia:  ;--tw-contain-size:  ;--tw-contain-layout:  ;--tw-contain-paint:  ;--tw-contain-style:  }.svelte-7w3m9l::backdrop{--tw-border-spacing-x:0;--tw-border-spacing-y:0;--tw-translate-x:0;--tw-translate-y:0;--tw-rotate:0;--tw-skew-x:0;--tw-skew-y:0;--tw-scale-x:1;--tw-scale-y:1;--tw-pan-x:  ;--tw-pan-y:  ;--tw-pinch-zoom:  ;--tw-scroll-snap-strictness:proximity;--tw-gradient-from-position:  ;--tw-gradient-via-position:  ;--tw-gradient-to-position:  ;--tw-ordinal:  ;--tw-slashed-zero:  ;--tw-numeric-figure:  ;--tw-numeric-spacing:  ;--tw-numeric-fraction:  ;--tw-ring-inset:  ;--tw-ring-offset-width:0px;--tw-ring-offset-color:#fff;--tw-ring-color:rgb(59 130 246 / 0.5);--tw-ring-offset-shadow:0 0 #0000;--tw-ring-shadow:0 0 #0000;--tw-shadow:0 0 #0000;--tw-shadow-colored:0 0 #0000;--tw-blur:  ;--tw-brightness:  ;--tw-contrast:  ;--tw-grayscale:  ;--tw-hue-rotate:  ;--tw-invert:  ;--tw-saturate:  ;--tw-sepia:  ;--tw-drop-shadow:  ;--tw-backdrop-blur:  ;--tw-backdrop-brightness:  ;--tw-backdrop-contrast:  ;--tw-backdrop-grayscale:  ;--tw-backdrop-hue-rotate:  ;--tw-backdrop-invert:  ;--tw-backdrop-opacity:  ;--tw-backdrop-saturate:  ;--tw-backdrop-sepia:  ;--tw-contain-size:  ;--tw-contain-layout:  ;--tw-contain-paint:  ;--tw-contain-style:  }@media(min-width: 640px){}@media(min-width: 768px){}@media(min-width: 1024px){}@media(min-width: 1280px){}@media(min-width: 1536px){}.fixed.svelte-7w3m9l{position:fixed}.left-0.svelte-7w3m9l{left:0px}.right-0.svelte-7w3m9l{right:0px}.top-0.svelte-7w3m9l{top:0px}.mx-4.svelte-7w3m9l{margin-left:1rem;margin-right:1rem}.mb-2.svelte-7w3m9l{margin-bottom:0.5rem}.mt-4.svelte-7w3m9l{margin-top:1rem}.flex.svelte-7w3m9l{display:flex}.h-screen.svelte-7w3m9l{height:100vh}.w-64.svelte-7w3m9l{width:16rem}.flex-col.svelte-7w3m9l{flex-direction:column}.overflow-hidden.svelte-7w3m9l{overflow:hidden}.rounded-lg.svelte-7w3m9l{border-radius:0.5rem}.bg-primary-400.svelte-7w3m9l{--tw-bg-opacity:1;background-color:rgb(69 137 255 / var(--tw-bg-opacity))}.bg-primary-500.svelte-7w3m9l{--tw-bg-opacity:1;background-color:rgb(15 98 254 / var(--tw-bg-opacity))}.px-4.svelte-7w3m9l{padding-left:1rem;padding-right:1rem}.px-5.svelte-7w3m9l{padding-left:1.25rem;padding-right:1.25rem}.py-2.svelte-7w3m9l{padding-top:0.5rem;padding-bottom:0.5rem}.py-2\\.5.svelte-7w3m9l{padding-top:0.625rem;padding-bottom:0.625rem}.pl-64.svelte-7w3m9l{padding-left:16rem}.text-center.svelte-7w3m9l{text-align:center}.text-sm.svelte-7w3m9l{font-size:0.875rem;line-height:1.25rem}.font-medium.svelte-7w3m9l{font-weight:500}.font-semibold.svelte-7w3m9l{font-weight:600}.text-light.svelte-7w3m9l{--tw-text-opacity:1;color:rgb(243 244 246 / var(--tw-text-opacity))}body{--tw-bg-opacity:1;background-color:rgb(244 244 245 / var(--tw-bg-opacity));--tw-text-opacity:1;color:rgb(17 24 39 / var(--tw-text-opacity))}body.dark{--tw-bg-opacity:1;background-color:rgb(63 63 70 / var(--tw-bg-opacity));--tw-text-opacity:1;color:rgb(243 244 246 / var(--tw-text-opacity))}.hover\\:bg-primary-200.svelte-7w3m9l:hover{--tw-bg-opacity:1;background-color:rgb(166 200 255 / var(--tw-bg-opacity))}.hover\\:bg-primary-400.svelte-7w3m9l:hover{--tw-bg-opacity:1;background-color:rgb(69 137 255 / var(--tw-bg-opacity))}.active\\:bg-primary-300.svelte-7w3m9l:active{--tw-bg-opacity:1;background-color:rgb(120 169 255 / var(--tw-bg-opacity))}.dark\\:bg-darkbackground-500.svelte-7w3m9l:is(.dark *){--tw-bg-opacity:1;background-color:rgb(63 63 70 / var(--tw-bg-opacity))}.dark\\:bg-primary-300.svelte-7w3m9l:is(.dark *){--tw-bg-opacity:1;background-color:rgb(120 169 255 / var(--tw-bg-opacity))}.dark\\:hover\\:bg-primary-200.svelte-7w3m9l:hover:is(.dark *){--tw-bg-opacity:1;background-color:rgb(166 200 255 / var(--tw-bg-opacity))}`,
  map: `{"version":3,"file":"+page.svelte","sources":["+page.svelte"],"sourcesContent":["<script>\\n\\timport { listen } from \\"@tauri-apps/api/event\\";\\n\\timport { open, save } from '@tauri-apps/plugin-dialog';\\n\\n\\timport { invoke } from \\"@tauri-apps/api/core\\"\\n\\timport { onMount, onDestroy } from \\"svelte\\";\\n\\n\\timport { path, form_inputs, dark} from '../stores';\\n\\t\\n\\timport Dashboard from '../pages/Dashboard.svelte';\\n\\timport Settings from '../pages/Settings.svelte';\\n\\timport College from '../pages/College.svelte';\\n\\timport Expenses from '../pages/Expenses.svelte';\\n\\timport Hsa from '../pages/Hsa.svelte';\\n\\timport Income from '../pages/Income.svelte';\\n\\timport Loan from '../pages/Loan.svelte';\\n\\timport Mortgage from '../pages/Mortgage.svelte';\\n\\timport Retirement from '../pages/Retirement.svelte';\\n\\timport Savings from '../pages/Savings.svelte';\\n\\timport Ssa from '../pages/Ssa.svelte';\\n\\t\\n\\tconst pages = [\\n\\t\\t{text: 'Dashboard', value: Dashboard, to: 'Dashboard'},\\n\\t\\t{text: 'Settings', value: Settings, to: 'Settings'},\\n\\t\\t{text: 'College', value: College, to: 'College'},\\n\\t\\t{text: 'Expenses', value: Expenses, to: 'Expenses'},\\n\\t\\t{text: 'HSA', value: Hsa, to: 'Hsa'},\\n\\t\\t{text: 'Income', value: Income, to: 'Income'},\\n\\t\\t{text: 'Loan', value: Loan, to: 'Loan'},\\n\\t\\t{text: 'Mortgage', value: Mortgage, to: 'Mortgage'},\\n\\t\\t{text: 'Retirement', value: Retirement, to: 'Retirement'},\\n\\t\\t{text: 'Savings', value: Savings, to: 'Savings'},\\n\\t\\t{text: 'SSA', value: Ssa, to: 'Ssa'},\\n\\t];\\n\\tlet selected = pages[0];\\n\\n\\tfunction openFile(pathString) {\\n\\t\\tinvoke(\\"file_open\\", {\\n\\t\\t\\tpath: pathString,\\n\\t\\t})\\n\\t\\t.then((data) => {\\n\\t\\t\\tform_inputs.set(data);\\n\\t\\t})\\n\\t\\t.catch((error) => alert(error));\\n\\t}\\n\\n\\tfunction saveFile(pathString, data) {\\n\\t\\tinvoke(\\"file_save\\", {\\n\\t\\t\\tpath: pathString,\\n\\t\\t\\tdata: data,\\n\\t\\t})\\n\\t\\t.catch((error) => alert(error));\\n\\t}\\n\\t\\n\\tlet unlisten;\\n\\tonMount(async () => {\\n\\t\\tform_inputs.reset();\\n\\n\\t\\tconsole.log(\\"listening to rust-event\\");\\n\\n\\t\\tunlisten = await listen('rust-event', (event) => {\\n\\t\\t\\tconsole.log(event);\\n\\n\\t\\t\\tswitch (event.payload.name) {\\n\\t\\t\\t\\tcase 'file-open' :\\n\\t\\t\\t\\t\\tconsole.log(\\"open\\");\\n\\t\\t\\t\\t\\topen()\\n\\t\\t\\t\\t\\t.then(function (pathString) {\\n\\t\\t\\t\\t\\t\\tif (pathString) {\\n\\t\\t\\t\\t\\t\\t\\t// @ts-ignore\\n\\t\\t\\t\\t\\t\\t\\tpath.set(pathString);\\n\\t\\t\\t\\t\\t\\t\\topenFile($path);\\n\\t\\t\\t\\t\\t\\t}\\n\\t\\t\\t\\t\\t});\\n\\t\\t\\t\\t\\tbreak;\\n\\t\\t\\t\\tcase 'file-save' :\\n\\t\\t\\t\\t\\tsaveFile($path, $form_inputs);\\n\\t\\t\\t\\t\\tbreak;\\n\\t\\t\\t\\tcase 'file-save_as' :\\n\\t\\t\\t\\t\\tsave()\\n\\t\\t\\t\\t\\t.then(function (pathString) {\\n\\t\\t\\t\\t\\t\\tif (pathString) {\\n\\t\\t\\t\\t\\t\\t\\tpath.set(pathString);\\n\\t\\t\\t\\t\\t\\t\\tsaveFile($path, $form_inputs);\\n\\t\\t\\t\\t\\t\\t}\\n\\t\\t\\t\\t\\t});\\n\\t\\t\\t\\t\\tbreak;\\n\\t\\t\\t\\tdefault : \\n\\t\\t\\t\\t\\talert(\\"not sure what to do\\");\\n\\t\\t\\t}\\n\\t\\t})\\n\\t})\\n\\t\\n\\tonDestroy(() => {\\n\\t\\tif (unlisten) {\\n\\t\\t\\tunlisten()\\n\\t\\t}\\n\\t})\\n\\n\\tfunction toggleDark() {\\n\\t\\tdark.set(!$dark);\\n\\t\\twindow.document.body.classList.toggle('dark')\\n\\t}\\n\\t\\t\\t\\t\\t\\t\\n<\/script>\\n\\n<aside class=\\"top-0 left-0 w-64 h-screen fixed bg-background-200 dark:bg-darkbackground-500\\">\\n\\t\\t<ul class=\\"flex flex-col overflow-hidden\\">\\n\\t\\t\\t{#each pages as page}\\n\\t\\t\\t\\t<li \\n\\t\\t\\t\\t\\ton:click={() => {selected = page;}}\\n\\t\\t\\t\\t\\ton:keypress={() => {}}\\n\\t\\t\\t\\t\\tclass=\\"hover:bg-primary-200 active:bg-primary-300 py-2 px-4 font-semibold\\"\\n\\t\\t\\t\\t\\tclass:bg-primary-400={selected === page}\\n\\t\\t\\t\\t>\\n\\t\\t\\t\\t\\t{page.text}\\n\\t\\t\\t\\t</li>\\n\\t\\t\\t{/each}\\n\\t\\t</ul>\\n\\t\\t<hr />\\n\\t\\t<button \\n\\t\\t\\tclass=\\"text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 mt-4 dark:bg-primary-300 dark:hover:bg-primary-200 \\"\\n\\t\\t\\ton:click={toggleDark}\\n\\t\\t>\\n\\t\\t\\tToggle Dark Mode\\n\\t\\t</button>\\n</aside>\\n<main class=\\"top-0 right-0 pl-64 mx-4\\">\\n\\t<svelte:component this={selected.value}/>\\n</main>\\n\\n<!-- You can put your \\"global\\" style configurations here! -->\\n<style global lang=\\"postcss\\">/*\\n! tailwindcss v3.4.4 | MIT License | https://tailwindcss.com\\n*//*\\n1. Prevent padding and border from affecting element width. (https://github.com/mozdevs/cssremedy/issues/4)\\n2. Allow adding a border to an element by just adding a border-width. (https://github.com/tailwindcss/tailwindcss/pull/116)\\n*/\\n\\n*,\\n::before,\\n::after {\\n  box-sizing: border-box; /* 1 */\\n  border-width: 0; /* 2 */\\n  border-style: solid; /* 2 */\\n  border-color: currentColor; /* 2 */\\n}\\n\\n::before,\\n::after {\\n  --tw-content: '';\\n}\\n\\n/*\\n1. Use a consistent sensible line-height in all browsers.\\n2. Prevent adjustments of font size after orientation changes in iOS.\\n3. Use a more readable tab size.\\n4. Use the user's configured \`sans\` font-family by default.\\n5. Use the user's configured \`sans\` font-feature-settings by default.\\n6. Use the user's configured \`sans\` font-variation-settings by default.\\n7. Disable tap highlights on iOS\\n*/\\n\\nhtml,\\n:host {\\n  line-height: 1.5; /* 1 */\\n  -webkit-text-size-adjust: 100%; /* 2 */\\n  -moz-tab-size: 4; /* 3 */\\n  -o-tab-size: 4;\\n     tab-size: 4; /* 3 */\\n  font-family: ui-sans-serif, system-ui, sans-serif, \\"Apple Color Emoji\\", \\"Segoe UI Emoji\\", \\"Segoe UI Symbol\\", \\"Noto Color Emoji\\"; /* 4 */\\n  font-feature-settings: normal; /* 5 */\\n  font-variation-settings: normal; /* 6 */\\n  -webkit-tap-highlight-color: transparent; /* 7 */\\n}\\n\\n/*\\n1. Remove the margin in all browsers.\\n2. Inherit line-height from \`html\` so users can set them as a class directly on the \`html\` element.\\n*/\\n\\nbody {\\n  margin: 0; /* 1 */\\n  line-height: inherit; /* 2 */\\n}\\n\\n/*\\n1. Add the correct height in Firefox.\\n2. Correct the inheritance of border color in Firefox. (https://bugzilla.mozilla.org/show_bug.cgi?id=190655)\\n3. Ensure horizontal rules are visible by default.\\n*/\\n\\nhr {\\n  height: 0; /* 1 */\\n  color: inherit; /* 2 */\\n  border-top-width: 1px; /* 3 */\\n}\\n\\n/*\\nAdd the correct text decoration in Chrome, Edge, and Safari.\\n*/\\n\\nabbr:where([title]) {\\n  -webkit-text-decoration: underline dotted;\\n          text-decoration: underline dotted;\\n}\\n\\n/*\\nRemove the default font size and weight for headings.\\n*/\\n\\nh1,\\nh2,\\nh3,\\nh4,\\nh5,\\nh6 {\\n  font-size: inherit;\\n  font-weight: inherit;\\n}\\n\\n/*\\nReset links to optimize for opt-in styling instead of opt-out.\\n*/\\n\\na {\\n  color: inherit;\\n  text-decoration: inherit;\\n}\\n\\n/*\\nAdd the correct font weight in Edge and Safari.\\n*/\\n\\nb,\\nstrong {\\n  font-weight: bolder;\\n}\\n\\n/*\\n1. Use the user's configured \`mono\` font-family by default.\\n2. Use the user's configured \`mono\` font-feature-settings by default.\\n3. Use the user's configured \`mono\` font-variation-settings by default.\\n4. Correct the odd \`em\` font sizing in all browsers.\\n*/\\n\\ncode,\\nkbd,\\nsamp,\\npre {\\n  font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, \\"Liberation Mono\\", \\"Courier New\\", monospace; /* 1 */\\n  font-feature-settings: normal; /* 2 */\\n  font-variation-settings: normal; /* 3 */\\n  font-size: 1em; /* 4 */\\n}\\n\\n/*\\nAdd the correct font size in all browsers.\\n*/\\n\\nsmall {\\n  font-size: 80%;\\n}\\n\\n/*\\nPrevent \`sub\` and \`sup\` elements from affecting the line height in all browsers.\\n*/\\n\\nsub,\\nsup {\\n  font-size: 75%;\\n  line-height: 0;\\n  position: relative;\\n  vertical-align: baseline;\\n}\\n\\nsub {\\n  bottom: -0.25em;\\n}\\n\\nsup {\\n  top: -0.5em;\\n}\\n\\n/*\\n1. Remove text indentation from table contents in Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=999088, https://bugs.webkit.org/show_bug.cgi?id=201297)\\n2. Correct table border color inheritance in all Chrome and Safari. (https://bugs.chromium.org/p/chromium/issues/detail?id=935729, https://bugs.webkit.org/show_bug.cgi?id=195016)\\n3. Remove gaps between table borders by default.\\n*/\\n\\ntable {\\n  text-indent: 0; /* 1 */\\n  border-color: inherit; /* 2 */\\n  border-collapse: collapse; /* 3 */\\n}\\n\\n/*\\n1. Change the font styles in all browsers.\\n2. Remove the margin in Firefox and Safari.\\n3. Remove default padding in all browsers.\\n*/\\n\\nbutton,\\ninput,\\noptgroup,\\nselect,\\ntextarea {\\n  font-family: inherit; /* 1 */\\n  font-feature-settings: inherit; /* 1 */\\n  font-variation-settings: inherit; /* 1 */\\n  font-size: 100%; /* 1 */\\n  font-weight: inherit; /* 1 */\\n  line-height: inherit; /* 1 */\\n  letter-spacing: inherit; /* 1 */\\n  color: inherit; /* 1 */\\n  margin: 0; /* 2 */\\n  padding: 0; /* 3 */\\n}\\n\\n/*\\nRemove the inheritance of text transform in Edge and Firefox.\\n*/\\n\\nbutton,\\nselect {\\n  text-transform: none;\\n}\\n\\n/*\\n1. Correct the inability to style clickable types in iOS and Safari.\\n2. Remove default button styles.\\n*/\\n\\nbutton,\\ninput:where([type='button']),\\ninput:where([type='reset']),\\ninput:where([type='submit']) {\\n  -webkit-appearance: button; /* 1 */\\n  background-color: transparent; /* 2 */\\n  background-image: none; /* 2 */\\n}\\n\\n/*\\nUse the modern Firefox focus style for all focusable elements.\\n*/\\n\\n:-moz-focusring {\\n  outline: auto;\\n}\\n\\n/*\\nRemove the additional \`:invalid\` styles in Firefox. (https://github.com/mozilla/gecko-dev/blob/2f9eacd9d3d995c937b4251a5557d95d494c9be1/layout/style/res/forms.css#L728-L737)\\n*/\\n\\n:-moz-ui-invalid {\\n  box-shadow: none;\\n}\\n\\n/*\\nAdd the correct vertical alignment in Chrome and Firefox.\\n*/\\n\\nprogress {\\n  vertical-align: baseline;\\n}\\n\\n/*\\nCorrect the cursor style of increment and decrement buttons in Safari.\\n*/\\n\\n::-webkit-inner-spin-button,\\n::-webkit-outer-spin-button {\\n  height: auto;\\n}\\n\\n/*\\n1. Correct the odd appearance in Chrome and Safari.\\n2. Correct the outline style in Safari.\\n*/\\n\\n[type='search'] {\\n  -webkit-appearance: textfield; /* 1 */\\n  outline-offset: -2px; /* 2 */\\n}\\n\\n/*\\nRemove the inner padding in Chrome and Safari on macOS.\\n*/\\n\\n::-webkit-search-decoration {\\n  -webkit-appearance: none;\\n}\\n\\n/*\\n1. Correct the inability to style clickable types in iOS and Safari.\\n2. Change font properties to \`inherit\` in Safari.\\n*/\\n\\n::-webkit-file-upload-button {\\n  -webkit-appearance: button; /* 1 */\\n  font: inherit; /* 2 */\\n}\\n\\n/*\\nAdd the correct display in Chrome and Safari.\\n*/\\n\\nsummary {\\n  display: list-item;\\n}\\n\\n/*\\nRemoves the default spacing and border for appropriate elements.\\n*/\\n\\nblockquote,\\ndl,\\ndd,\\nh1,\\nh2,\\nh3,\\nh4,\\nh5,\\nh6,\\nhr,\\nfigure,\\np,\\npre {\\n  margin: 0;\\n}\\n\\nfieldset {\\n  margin: 0;\\n  padding: 0;\\n}\\n\\nlegend {\\n  padding: 0;\\n}\\n\\nol,\\nul,\\nmenu {\\n  list-style: none;\\n  margin: 0;\\n  padding: 0;\\n}\\n\\n/*\\nReset default styling for dialogs.\\n*/\\ndialog {\\n  padding: 0;\\n}\\n\\n/*\\nPrevent resizing textareas horizontally by default.\\n*/\\n\\ntextarea {\\n  resize: vertical;\\n}\\n\\n/*\\n1. Reset the default placeholder opacity in Firefox. (https://github.com/tailwindlabs/tailwindcss/issues/3300)\\n2. Set the default placeholder color to the user's configured gray 400 color.\\n*/\\n\\ninput::-moz-placeholder, textarea::-moz-placeholder {\\n  opacity: 1; /* 1 */\\n  color: #9ca3af; /* 2 */\\n}\\n\\ninput::placeholder,\\ntextarea::placeholder {\\n  opacity: 1; /* 1 */\\n  color: #9ca3af; /* 2 */\\n}\\n\\n/*\\nSet the default cursor for buttons.\\n*/\\n\\nbutton,\\n[role=\\"button\\"] {\\n  cursor: pointer;\\n}\\n\\n/*\\nMake sure disabled buttons don't get the pointer cursor.\\n*/\\n:disabled {\\n  cursor: default;\\n}\\n\\n/*\\n1. Make replaced elements \`display: block\` by default. (https://github.com/mozdevs/cssremedy/issues/14)\\n2. Add \`vertical-align: middle\` to align replaced elements more sensibly by default. (https://github.com/jensimmons/cssremedy/issues/14#issuecomment-634934210)\\n   This can trigger a poorly considered lint error in some tools but is included by design.\\n*/\\n\\nimg,\\nsvg,\\nvideo,\\ncanvas,\\naudio,\\niframe,\\nembed,\\nobject {\\n  display: block; /* 1 */\\n  vertical-align: middle; /* 2 */\\n}\\n\\n/*\\nConstrain images and videos to the parent width and preserve their intrinsic aspect ratio. (https://github.com/mozdevs/cssremedy/issues/14)\\n*/\\n\\nimg,\\nvideo {\\n  max-width: 100%;\\n  height: auto;\\n}\\n\\n/* Make elements with the HTML hidden attribute stay hidden by default */\\n[hidden] {\\n  display: none;\\n}\\n\\n*, ::before, ::after {\\n  --tw-border-spacing-x: 0;\\n  --tw-border-spacing-y: 0;\\n  --tw-translate-x: 0;\\n  --tw-translate-y: 0;\\n  --tw-rotate: 0;\\n  --tw-skew-x: 0;\\n  --tw-skew-y: 0;\\n  --tw-scale-x: 1;\\n  --tw-scale-y: 1;\\n  --tw-pan-x:  ;\\n  --tw-pan-y:  ;\\n  --tw-pinch-zoom:  ;\\n  --tw-scroll-snap-strictness: proximity;\\n  --tw-gradient-from-position:  ;\\n  --tw-gradient-via-position:  ;\\n  --tw-gradient-to-position:  ;\\n  --tw-ordinal:  ;\\n  --tw-slashed-zero:  ;\\n  --tw-numeric-figure:  ;\\n  --tw-numeric-spacing:  ;\\n  --tw-numeric-fraction:  ;\\n  --tw-ring-inset:  ;\\n  --tw-ring-offset-width: 0px;\\n  --tw-ring-offset-color: #fff;\\n  --tw-ring-color: rgb(59 130 246 / 0.5);\\n  --tw-ring-offset-shadow: 0 0 #0000;\\n  --tw-ring-shadow: 0 0 #0000;\\n  --tw-shadow: 0 0 #0000;\\n  --tw-shadow-colored: 0 0 #0000;\\n  --tw-blur:  ;\\n  --tw-brightness:  ;\\n  --tw-contrast:  ;\\n  --tw-grayscale:  ;\\n  --tw-hue-rotate:  ;\\n  --tw-invert:  ;\\n  --tw-saturate:  ;\\n  --tw-sepia:  ;\\n  --tw-drop-shadow:  ;\\n  --tw-backdrop-blur:  ;\\n  --tw-backdrop-brightness:  ;\\n  --tw-backdrop-contrast:  ;\\n  --tw-backdrop-grayscale:  ;\\n  --tw-backdrop-hue-rotate:  ;\\n  --tw-backdrop-invert:  ;\\n  --tw-backdrop-opacity:  ;\\n  --tw-backdrop-saturate:  ;\\n  --tw-backdrop-sepia:  ;\\n  --tw-contain-size:  ;\\n  --tw-contain-layout:  ;\\n  --tw-contain-paint:  ;\\n  --tw-contain-style:  ;\\n}\\n\\n::backdrop {\\n  --tw-border-spacing-x: 0;\\n  --tw-border-spacing-y: 0;\\n  --tw-translate-x: 0;\\n  --tw-translate-y: 0;\\n  --tw-rotate: 0;\\n  --tw-skew-x: 0;\\n  --tw-skew-y: 0;\\n  --tw-scale-x: 1;\\n  --tw-scale-y: 1;\\n  --tw-pan-x:  ;\\n  --tw-pan-y:  ;\\n  --tw-pinch-zoom:  ;\\n  --tw-scroll-snap-strictness: proximity;\\n  --tw-gradient-from-position:  ;\\n  --tw-gradient-via-position:  ;\\n  --tw-gradient-to-position:  ;\\n  --tw-ordinal:  ;\\n  --tw-slashed-zero:  ;\\n  --tw-numeric-figure:  ;\\n  --tw-numeric-spacing:  ;\\n  --tw-numeric-fraction:  ;\\n  --tw-ring-inset:  ;\\n  --tw-ring-offset-width: 0px;\\n  --tw-ring-offset-color: #fff;\\n  --tw-ring-color: rgb(59 130 246 / 0.5);\\n  --tw-ring-offset-shadow: 0 0 #0000;\\n  --tw-ring-shadow: 0 0 #0000;\\n  --tw-shadow: 0 0 #0000;\\n  --tw-shadow-colored: 0 0 #0000;\\n  --tw-blur:  ;\\n  --tw-brightness:  ;\\n  --tw-contrast:  ;\\n  --tw-grayscale:  ;\\n  --tw-hue-rotate:  ;\\n  --tw-invert:  ;\\n  --tw-saturate:  ;\\n  --tw-sepia:  ;\\n  --tw-drop-shadow:  ;\\n  --tw-backdrop-blur:  ;\\n  --tw-backdrop-brightness:  ;\\n  --tw-backdrop-contrast:  ;\\n  --tw-backdrop-grayscale:  ;\\n  --tw-backdrop-hue-rotate:  ;\\n  --tw-backdrop-invert:  ;\\n  --tw-backdrop-opacity:  ;\\n  --tw-backdrop-saturate:  ;\\n  --tw-backdrop-sepia:  ;\\n  --tw-contain-size:  ;\\n  --tw-contain-layout:  ;\\n  --tw-contain-paint:  ;\\n  --tw-contain-style:  ;\\n}\\n    .container {\\n  width: 100%;\\n}\\n    @media (min-width: 640px) {\\n\\n  .container {\\n    max-width: 640px;\\n  }\\n}\\n    @media (min-width: 768px) {\\n\\n  .container {\\n    max-width: 768px;\\n  }\\n}\\n    @media (min-width: 1024px) {\\n\\n  .container {\\n    max-width: 1024px;\\n  }\\n}\\n    @media (min-width: 1280px) {\\n\\n  .container {\\n    max-width: 1280px;\\n  }\\n}\\n    @media (min-width: 1536px) {\\n\\n  .container {\\n    max-width: 1536px;\\n  }\\n}\\n    .sr-only {\\n  position: absolute;\\n  width: 1px;\\n  height: 1px;\\n  padding: 0;\\n  margin: -1px;\\n  overflow: hidden;\\n  clip: rect(0, 0, 0, 0);\\n  white-space: nowrap;\\n  border-width: 0;\\n}\\n    .visible {\\n  visibility: visible;\\n}\\n    .invisible {\\n  visibility: hidden;\\n}\\n    .fixed {\\n  position: fixed;\\n}\\n    .absolute {\\n  position: absolute;\\n}\\n    .relative {\\n  position: relative;\\n}\\n    .inset-0 {\\n  inset: 0px;\\n}\\n    .-left-1 {\\n  left: -0.25rem;\\n}\\n    .-top-1 {\\n  top: -0.25rem;\\n}\\n    .left-0 {\\n  left: 0px;\\n}\\n    .right-0 {\\n  right: 0px;\\n}\\n    .top-0 {\\n  top: 0px;\\n}\\n    .col-span-10 {\\n  grid-column: span 10 / span 10;\\n}\\n    .col-span-3 {\\n  grid-column: span 3 / span 3;\\n}\\n    .col-span-5 {\\n  grid-column: span 5 / span 5;\\n}\\n    .col-span-7 {\\n  grid-column: span 7 / span 7;\\n}\\n    .m-0 {\\n  margin: 0px;\\n}\\n    .m-auto {\\n  margin: auto;\\n}\\n    .mx-2 {\\n  margin-left: 0.5rem;\\n  margin-right: 0.5rem;\\n}\\n    .mx-32 {\\n  margin-left: 8rem;\\n  margin-right: 8rem;\\n}\\n    .mx-4 {\\n  margin-left: 1rem;\\n  margin-right: 1rem;\\n}\\n    .mx-auto {\\n  margin-left: auto;\\n  margin-right: auto;\\n}\\n    .my-1 {\\n  margin-top: 0.25rem;\\n  margin-bottom: 0.25rem;\\n}\\n    .-mt-10 {\\n  margin-top: -2.5rem;\\n}\\n    .mb-2 {\\n  margin-bottom: 0.5rem;\\n}\\n    .ml-2 {\\n  margin-left: 0.5rem;\\n}\\n    .ml-4 {\\n  margin-left: 1rem;\\n}\\n    .ml-6 {\\n  margin-left: 1.5rem;\\n}\\n    .mr-2 {\\n  margin-right: 0.5rem;\\n}\\n    .mr-3 {\\n  margin-right: 0.75rem;\\n}\\n    .mr-4 {\\n  margin-right: 1rem;\\n}\\n    .mt-4 {\\n  margin-top: 1rem;\\n}\\n    .mt-6 {\\n  margin-top: 1.5rem;\\n}\\n    .flex {\\n  display: flex;\\n}\\n    .table {\\n  display: table;\\n}\\n    .grid {\\n  display: grid;\\n}\\n    .contents {\\n  display: contents;\\n}\\n    .hidden {\\n  display: none;\\n}\\n    .h-2 {\\n  height: 0.5rem;\\n}\\n    .h-4 {\\n  height: 1rem;\\n}\\n    .h-48 {\\n  height: 12rem;\\n}\\n    .h-6 {\\n  height: 1.5rem;\\n}\\n    .h-fit {\\n  height: -moz-fit-content;\\n  height: fit-content;\\n}\\n    .h-full {\\n  height: 100%;\\n}\\n    .h-screen {\\n  height: 100vh;\\n}\\n    .w-10 {\\n  width: 2.5rem;\\n}\\n    .w-16 {\\n  width: 4rem;\\n}\\n    .w-32 {\\n  width: 8rem;\\n}\\n    .w-4 {\\n  width: 1rem;\\n}\\n    .w-6 {\\n  width: 1.5rem;\\n}\\n    .w-64 {\\n  width: 16rem;\\n}\\n    .w-\\\\[32rem\\\\] {\\n  width: 32rem;\\n}\\n    .w-fit {\\n  width: -moz-fit-content;\\n  width: fit-content;\\n}\\n    .w-full {\\n  width: 100%;\\n}\\n    .w-screen {\\n  width: 100vw;\\n}\\n    .flex-auto {\\n  flex: 1 1 auto;\\n}\\n    .flex-initial {\\n  flex: 0 1 auto;\\n}\\n    .grow {\\n  flex-grow: 1;\\n}\\n    .table-auto {\\n  table-layout: auto;\\n}\\n    .transform {\\n  transform: translate(var(--tw-translate-x), var(--tw-translate-y)) rotate(var(--tw-rotate)) skewX(var(--tw-skew-x)) skewY(var(--tw-skew-y)) scaleX(var(--tw-scale-x)) scaleY(var(--tw-scale-y));\\n}\\n    .cursor-pointer {\\n  cursor: pointer;\\n}\\n    .columns-1 {\\n  -moz-columns: 1;\\n       columns: 1;\\n}\\n    .grid-cols-1 {\\n  grid-template-columns: repeat(1, minmax(0, 1fr));\\n}\\n    .grid-cols-10 {\\n  grid-template-columns: repeat(10, minmax(0, 1fr));\\n}\\n    .grid-cols-2 {\\n  grid-template-columns: repeat(2, minmax(0, 1fr));\\n}\\n    .grid-cols-3 {\\n  grid-template-columns: repeat(3, minmax(0, 1fr));\\n}\\n    .grid-rows-1 {\\n  grid-template-rows: repeat(1, minmax(0, 1fr));\\n}\\n    .flex-col {\\n  flex-direction: column;\\n}\\n    .flex-wrap {\\n  flex-wrap: wrap;\\n}\\n    .place-content-center {\\n  place-content: center;\\n}\\n    .place-content-start {\\n  place-content: start;\\n}\\n    .place-content-end {\\n  place-content: end;\\n}\\n    .content-center {\\n  align-content: center;\\n}\\n    .items-center {\\n  align-items: center;\\n}\\n    .gap-1 {\\n  gap: 0.25rem;\\n}\\n    .gap-2 {\\n  gap: 0.5rem;\\n}\\n    .gap-4 {\\n  gap: 1rem;\\n}\\n    .overflow-hidden {\\n  overflow: hidden;\\n}\\n    .overflow-visible {\\n  overflow: visible;\\n}\\n    .rounded {\\n  border-radius: 0.25rem;\\n}\\n    .rounded-full {\\n  border-radius: 9999px;\\n}\\n    .rounded-lg {\\n  border-radius: 0.5rem;\\n}\\n    .rounded-sm {\\n  border-radius: 0.125rem;\\n}\\n    .rounded-t {\\n  border-top-left-radius: 0.25rem;\\n  border-top-right-radius: 0.25rem;\\n}\\n    .border {\\n  border-width: 1px;\\n}\\n    .border-dark {\\n  --tw-border-opacity: 1;\\n  border-color: rgb(17 24 39 / var(--tw-border-opacity));\\n}\\n    .bg-background-400 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(250 250 250 / var(--tw-bg-opacity));\\n}\\n    .bg-background-500 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(244 244 245 / var(--tw-bg-opacity));\\n}\\n    .bg-background-600 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(228 228 231 / var(--tw-bg-opacity));\\n}\\n    .bg-black {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(0 0 0 / var(--tw-bg-opacity));\\n}\\n    .bg-primary-400 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(69 137 255 / var(--tw-bg-opacity));\\n}\\n    .bg-primary-500 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(15 98 254 / var(--tw-bg-opacity));\\n}\\n    .bg-secondary-500 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(113 113 122 / var(--tw-bg-opacity));\\n}\\n    .bg-warning-500 {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(245 158 11 / var(--tw-bg-opacity));\\n}\\n    .bg-white {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(255 255 255 / var(--tw-bg-opacity));\\n}\\n    .fill-background-500 {\\n  fill: #f4f4f5;\\n}\\n    .fill-background-700 {\\n  fill: #d4d4d8;\\n}\\n    .fill-dark {\\n  fill: #111827;\\n}\\n    .fill-graphics-1-600 {\\n  fill: hsl(216,83%,48%);\\n}\\n    .fill-graphics-4-400 {\\n  fill: hsl(345,71%,60%);\\n}\\n    .fill-graphics-7-100 {\\n  fill: hsl(32,82%,85%);\\n}\\n    .fill-graphics-7-50 {\\n  fill: hsl(32,82%,90%);\\n}\\n    .fill-primary-500 {\\n  fill: #0f62fe;\\n}\\n    .fill-secondary-100 {\\n  fill: #f4f4f5;\\n}\\n    .fill-secondary-50 {\\n  fill: #fafafa;\\n}\\n    .fill-secondary-500 {\\n  fill: #71717a;\\n}\\n    .stroke-background-800 {\\n  stroke: #a1a1aa;\\n}\\n    .stroke-dark {\\n  stroke: #111827;\\n}\\n    .stroke-graphics-1-500 {\\n  stroke: hsl(216,83%,53%);\\n}\\n    .stroke-graphics-4-800 {\\n  stroke: hsl(345,69%,32%);\\n}\\n    .p-0 {\\n  padding: 0px;\\n}\\n    .p-1 {\\n  padding: 0.25rem;\\n}\\n    .p-2 {\\n  padding: 0.5rem;\\n}\\n    .p-4 {\\n  padding: 1rem;\\n}\\n    .p-6 {\\n  padding: 1.5rem;\\n}\\n    .px-1 {\\n  padding-left: 0.25rem;\\n  padding-right: 0.25rem;\\n}\\n    .px-2 {\\n  padding-left: 0.5rem;\\n  padding-right: 0.5rem;\\n}\\n    .px-4 {\\n  padding-left: 1rem;\\n  padding-right: 1rem;\\n}\\n    .px-5 {\\n  padding-left: 1.25rem;\\n  padding-right: 1.25rem;\\n}\\n    .py-1 {\\n  padding-top: 0.25rem;\\n  padding-bottom: 0.25rem;\\n}\\n    .py-2 {\\n  padding-top: 0.5rem;\\n  padding-bottom: 0.5rem;\\n}\\n    .py-2\\\\.5 {\\n  padding-top: 0.625rem;\\n  padding-bottom: 0.625rem;\\n}\\n    .pl-1 {\\n  padding-left: 0.25rem;\\n}\\n    .pl-64 {\\n  padding-left: 16rem;\\n}\\n    .pr-2 {\\n  padding-right: 0.5rem;\\n}\\n    .pt-2 {\\n  padding-top: 0.5rem;\\n}\\n    .text-center {\\n  text-align: center;\\n}\\n    .align-middle {\\n  vertical-align: middle;\\n}\\n    .text-lg {\\n  font-size: 1.125rem;\\n  line-height: 1.75rem;\\n}\\n    .text-sm {\\n  font-size: 0.875rem;\\n  line-height: 1.25rem;\\n}\\n    .text-xl {\\n  font-size: 1.25rem;\\n  line-height: 1.75rem;\\n}\\n    .font-bold {\\n  font-weight: 700;\\n}\\n    .font-medium {\\n  font-weight: 500;\\n}\\n    .font-semibold {\\n  font-weight: 600;\\n}\\n    .text-dark {\\n  --tw-text-opacity: 1;\\n  color: rgb(17 24 39 / var(--tw-text-opacity));\\n}\\n    .text-light {\\n  --tw-text-opacity: 1;\\n  color: rgb(243 244 246 / var(--tw-text-opacity));\\n}\\n    .opacity-50 {\\n  opacity: 0.5;\\n}\\n    .shadow {\\n  --tw-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);\\n  --tw-shadow-colored: 0 1px 3px 0 var(--tw-shadow-color), 0 1px 2px -1px var(--tw-shadow-color);\\n  box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow);\\n}\\n    .shadow-inner {\\n  --tw-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);\\n  --tw-shadow-colored: inset 0 2px 4px 0 var(--tw-shadow-color);\\n  box-shadow: var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow);\\n}\\n    .outline {\\n  outline-style: solid;\\n}\\n    .invert {\\n  --tw-invert: invert(100%);\\n  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);\\n}\\n    .filter {\\n  filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);\\n}\\n    .transition {\\n  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, -webkit-backdrop-filter;\\n  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;\\n  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter, -webkit-backdrop-filter;\\n  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);\\n  transition-duration: 150ms;\\n}\\n\\n\\t:global(body) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(244 244 245 / var(--tw-bg-opacity));\\n  --tw-text-opacity: 1;\\n  color: rgb(17 24 39 / var(--tw-text-opacity));\\n}\\n\\t:global(body.dark) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(63 63 70 / var(--tw-bg-opacity));\\n  --tw-text-opacity: 1;\\n  color: rgb(243 244 246 / var(--tw-text-opacity));\\n}\\n\\t.hover\\\\:bg-primary-200:hover {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(166 200 255 / var(--tw-bg-opacity));\\n}\\n\\t.hover\\\\:bg-primary-400:hover {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(69 137 255 / var(--tw-bg-opacity));\\n}\\n\\t.hover\\\\:stroke-primary-500:hover {\\n  stroke: #0f62fe;\\n}\\n\\t.hover\\\\:stroke-secondary-500:hover {\\n  stroke: #71717a;\\n}\\n\\t.active\\\\:bg-primary-300:active {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(120 169 255 / var(--tw-bg-opacity));\\n}\\n\\t.disabled\\\\:bg-secondary-100:disabled {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(244 244 245 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:bg-darkbackground-300:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(113 113 122 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:bg-darkbackground-400:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(82 82 91 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:bg-darkbackground-500:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(63 63 70 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:bg-darkbackground-600:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(39 39 42 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:bg-primary-300:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(120 169 255 / var(--tw-bg-opacity));\\n}\\n\\t.dark\\\\:fill-darkbackground-500:is(.dark *) {\\n  fill: #3f3f46;\\n}\\n\\t.dark\\\\:fill-light:is(.dark *) {\\n  fill: #f3f4f6;\\n}\\n\\t.dark\\\\:text-light:is(.dark *) {\\n  --tw-text-opacity: 1;\\n  color: rgb(243 244 246 / var(--tw-text-opacity));\\n}\\n\\t.dark\\\\:hover\\\\:bg-primary-200:hover:is(.dark *) {\\n  --tw-bg-opacity: 1;\\n  background-color: rgb(166 200 255 / var(--tw-bg-opacity));\\n}\\n</style>"],"names":[],"mappings":"AA2IA,cAAC,eACD,QAAQ,eACR,OAAQ,CACN,UAAU,CAAE,UAAU,CACtB,YAAY,CAAE,CAAC,CACf,YAAY,CAAE,KAAK,CACnB,YAAY,CAAE,YAChB,eAEA,QAAQ,eACR,OAAQ,CACN,YAAY,CAAE,EAChB,CAaA,KAAM,CACJ,WAAW,CAAE,GAAG,CAChB,wBAAwB,CAAE,IAAI,CAC9B,aAAa,CAAE,CAAC,CAChB,WAAW,CAAE,CAAC,CACX,QAAQ,CAAE,CAAC,CACd,WAAW,CAAE,aAAa,CAAC,CAAC,SAAS,CAAC,CAAC,UAAU,CAAC,CAAC,mBAAmB,CAAC,CAAC,gBAAgB,CAAC,CAAC,iBAAiB,CAAC,CAAC,kBAAkB,CAC/H,qBAAqB,CAAE,MAAM,CAC7B,uBAAuB,CAAE,MAAM,CAC/B,2BAA2B,CAAE,WAC/B,CAkBA,gBAAG,CACD,MAAM,CAAE,CAAC,CACT,KAAK,CAAE,OAAO,CACd,gBAAgB,CAAE,GACpB,CA0GA,oBAIS,CACP,WAAW,CAAE,OAAO,CACpB,qBAAqB,CAAE,OAAO,CAC9B,uBAAuB,CAAE,OAAO,CAChC,SAAS,CAAE,IAAI,CACf,WAAW,CAAE,OAAO,CACpB,WAAW,CAAE,OAAO,CACpB,cAAc,CAAE,OAAO,CACvB,KAAK,CAAE,OAAO,CACd,MAAM,CAAE,CAAC,CACT,OAAO,CAAE,CACX,CAMA,oBACO,CACL,cAAc,CAAE,IAClB,CAOA,oBAG6B,CAC3B,kBAAkB,CAAE,MAAM,CAC1B,gBAAgB,CAAE,WAAW,CAC7B,gBAAgB,CAAE,IACpB,eAMA,eAAgB,CACd,OAAO,CAAE,IACX,eAMA,gBAAiB,CACf,UAAU,CAAE,IACd,eAcA,2BAA2B,eAC3B,2BAA4B,CAC1B,MAAM,CAAE,IACV,eAgBA,2BAA4B,CAC1B,kBAAkB,CAAE,IACtB,eAOA,4BAA6B,CAC3B,kBAAkB,CAAE,MAAM,CAC1B,IAAI,CAAE,OACR,CAuBA,gBAGI,CACF,MAAM,CAAE,CACV,CAYA,gBACK,CACH,UAAU,CAAE,IAAI,CAChB,MAAM,CAAE,CAAC,CACT,OAAO,CAAE,CACX,CAqCA,oBACgB,CACd,MAAM,CAAE,OACV,eAKA,SAAU,CACR,MAAM,CAAE,OACV,CAmCA,cAAC,eAAE,QAAQ,eAAE,OAAQ,CACnB,qBAAqB,CAAE,CAAC,CACxB,qBAAqB,CAAE,CAAC,CACxB,gBAAgB,CAAE,CAAC,CACnB,gBAAgB,CAAE,CAAC,CACnB,WAAW,CAAE,CAAC,CACd,WAAW,CAAE,CAAC,CACd,WAAW,CAAE,CAAC,CACd,YAAY,CAAE,CAAC,CACf,YAAY,CAAE,CAAC,CACf,WAAW,EAAE,CACb,WAAW,EAAE,CACb,gBAAgB,EAAE,CAClB,2BAA2B,CAAE,SAAS,CACtC,4BAA4B,EAAE,CAC9B,2BAA2B,EAAE,CAC7B,0BAA0B,EAAE,CAC5B,aAAa,EAAE,CACf,kBAAkB,EAAE,CACpB,oBAAoB,EAAE,CACtB,qBAAqB,EAAE,CACvB,sBAAsB,EAAE,CACxB,gBAAgB,EAAE,CAClB,sBAAsB,CAAE,GAAG,CAC3B,sBAAsB,CAAE,IAAI,CAC5B,eAAe,CAAE,qBAAqB,CACtC,uBAAuB,CAAE,SAAS,CAClC,gBAAgB,CAAE,SAAS,CAC3B,WAAW,CAAE,SAAS,CACtB,mBAAmB,CAAE,SAAS,CAC9B,UAAU,EAAE,CACZ,gBAAgB,EAAE,CAClB,cAAc,EAAE,CAChB,eAAe,EAAE,CACjB,gBAAgB,EAAE,CAClB,YAAY,EAAE,CACd,cAAc,EAAE,CAChB,WAAW,EAAE,CACb,iBAAiB,EAAE,CACnB,mBAAmB,EAAE,CACrB,yBAAyB,EAAE,CAC3B,uBAAuB,EAAE,CACzB,wBAAwB,EAAE,CAC1B,yBAAyB,EAAE,CAC3B,qBAAqB,EAAE,CACvB,sBAAsB,EAAE,CACxB,uBAAuB,EAAE,CACzB,oBAAoB,EAAE,CACtB,kBAAkB,EAAE,CACpB,oBAAoB,EAAE,CACtB,mBAAmB,EAAE,CACrB,mBAAmB,EACrB,eAEA,UAAW,CACT,qBAAqB,CAAE,CAAC,CACxB,qBAAqB,CAAE,CAAC,CACxB,gBAAgB,CAAE,CAAC,CACnB,gBAAgB,CAAE,CAAC,CACnB,WAAW,CAAE,CAAC,CACd,WAAW,CAAE,CAAC,CACd,WAAW,CAAE,CAAC,CACd,YAAY,CAAE,CAAC,CACf,YAAY,CAAE,CAAC,CACf,WAAW,EAAE,CACb,WAAW,EAAE,CACb,gBAAgB,EAAE,CAClB,2BAA2B,CAAE,SAAS,CACtC,4BAA4B,EAAE,CAC9B,2BAA2B,EAAE,CAC7B,0BAA0B,EAAE,CAC5B,aAAa,EAAE,CACf,kBAAkB,EAAE,CACpB,oBAAoB,EAAE,CACtB,qBAAqB,EAAE,CACvB,sBAAsB,EAAE,CACxB,gBAAgB,EAAE,CAClB,sBAAsB,CAAE,GAAG,CAC3B,sBAAsB,CAAE,IAAI,CAC5B,eAAe,CAAE,qBAAqB,CACtC,uBAAuB,CAAE,SAAS,CAClC,gBAAgB,CAAE,SAAS,CAC3B,WAAW,CAAE,SAAS,CACtB,mBAAmB,CAAE,SAAS,CAC9B,UAAU,EAAE,CACZ,gBAAgB,EAAE,CAClB,cAAc,EAAE,CAChB,eAAe,EAAE,CACjB,gBAAgB,EAAE,CAClB,YAAY,EAAE,CACd,cAAc,EAAE,CAChB,WAAW,EAAE,CACb,iBAAiB,EAAE,CACnB,mBAAmB,EAAE,CACrB,yBAAyB,EAAE,CAC3B,uBAAuB,EAAE,CACzB,wBAAwB,EAAE,CAC1B,yBAAyB,EAAE,CAC3B,qBAAqB,EAAE,CACvB,sBAAsB,EAAE,CACxB,uBAAuB,EAAE,CACzB,oBAAoB,EAAE,CACtB,kBAAkB,EAAE,CACpB,oBAAoB,EAAE,CACtB,mBAAmB,EAAE,CACrB,mBAAmB,EACrB,CAII,MAAO,YAAY,KAAK,CAAE,CAK9B,CACI,MAAO,YAAY,KAAK,CAAE,CAK9B,CACI,MAAO,YAAY,MAAM,CAAE,CAK/B,CACI,MAAO,YAAY,MAAM,CAAE,CAK/B,CACI,MAAO,YAAY,MAAM,CAAE,CAK/B,CAkBI,oBAAO,CACT,QAAQ,CAAE,KACZ,CAgBI,qBAAQ,CACV,IAAI,CAAE,GACR,CACI,sBAAS,CACX,KAAK,CAAE,GACT,CACI,oBAAO,CACT,GAAG,CAAE,GACP,CA2BI,mBAAM,CACR,WAAW,CAAE,IAAI,CACjB,YAAY,CAAE,IAChB,CAYI,mBAAM,CACR,aAAa,CAAE,MACjB,CAmBI,mBAAM,CACR,UAAU,CAAE,IACd,CAII,mBAAM,CACR,OAAO,CAAE,IACX,CAgCI,uBAAU,CACZ,MAAM,CAAE,KACV,CAgBI,mBAAM,CACR,KAAK,CAAE,KACT,CAmDI,uBAAU,CACZ,cAAc,CAAE,MAClB,CA4BI,8BAAiB,CACnB,QAAQ,CAAE,MACZ,CAUI,yBAAY,CACd,aAAa,CAAE,MACjB,CA+BI,6BAAgB,CAClB,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CACzD,CACI,6BAAgB,CAClB,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CACxD,CAiFI,mBAAM,CACR,YAAY,CAAE,IAAI,CAClB,aAAa,CAAE,IACjB,CACI,mBAAM,CACR,YAAY,CAAE,OAAO,CACrB,aAAa,CAAE,OACjB,CAKI,mBAAM,CACR,WAAW,CAAE,MAAM,CACnB,cAAc,CAAE,MAClB,CACI,sBAAS,CACX,WAAW,CAAE,QAAQ,CACrB,cAAc,CAAE,QAClB,CAII,oBAAO,CACT,YAAY,CAAE,KAChB,CAOI,0BAAa,CACf,UAAU,CAAE,MACd,CAQI,sBAAS,CACX,SAAS,CAAE,QAAQ,CACnB,WAAW,CAAE,OACf,CAQI,0BAAa,CACf,WAAW,CAAE,GACf,CACI,4BAAe,CACjB,WAAW,CAAE,GACf,CAKI,yBAAY,CACd,iBAAiB,CAAE,CAAC,CACpB,KAAK,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,iBAAiB,CAAC,CACjD,CAgCS,IAAM,CACb,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAAC,CACzD,iBAAiB,CAAE,CAAC,CACpB,KAAK,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,EAAE,CAAC,CAAC,CAAC,IAAI,iBAAiB,CAAC,CAC9C,CACS,SAAW,CAClB,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,EAAE,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAAC,CACtD,iBAAiB,CAAE,CAAC,CACpB,KAAK,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,iBAAiB,CAAC,CACjD,CACC,oCAAsB,MAAO,CAC5B,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D,CACC,oCAAsB,MAAO,CAC5B,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CACzD,CAOC,qCAAuB,OAAQ,CAC9B,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D,CAaC,0CAA4B,IAAI,KAAK,CAAC,CAAC,CAAE,CACxC,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,EAAE,CAAC,EAAE,CAAC,EAAE,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CACvD,CAKC,mCAAqB,IAAI,KAAK,CAAC,CAAC,CAAE,CACjC,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D,CAWC,0CAA4B,MAAM,IAAI,KAAK,CAAC,CAAC,CAAE,CAC9C,eAAe,CAAE,CAAC,CAClB,gBAAgB,CAAE,IAAI,GAAG,CAAC,GAAG,CAAC,GAAG,CAAC,CAAC,CAAC,IAAI,eAAe,CAAC,CAC1D"}`
};
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let $$unsubscribe_dark;
  let $$unsubscribe_form_inputs;
  let $$unsubscribe_path;
  $$unsubscribe_dark = subscribe(dark, (value) => value);
  $$unsubscribe_form_inputs = subscribe(form_inputs, (value) => value);
  $$unsubscribe_path = subscribe(path, (value) => value);
  const pages = [
    {
      text: "Dashboard",
      value: Dashboard,
      to: "Dashboard"
    },
    {
      text: "Settings",
      value: Settings,
      to: "Settings"
    },
    {
      text: "College",
      value: College,
      to: "College"
    },
    {
      text: "Expenses",
      value: Expenses,
      to: "Expenses"
    },
    { text: "HSA", value: Hsa, to: "Hsa" },
    {
      text: "Income",
      value: Income,
      to: "Income"
    },
    { text: "Loan", value: Loan, to: "Loan" },
    {
      text: "Mortgage",
      value: Mortgage,
      to: "Mortgage"
    },
    {
      text: "Retirement",
      value: Retirement,
      to: "Retirement"
    },
    {
      text: "Savings",
      value: Savings,
      to: "Savings"
    },
    { text: "SSA", value: Ssa, to: "Ssa" }
  ];
  let selected = pages[0];
  onDestroy(() => {
  });
  $$result.css.add(css);
  $$unsubscribe_dark();
  $$unsubscribe_form_inputs();
  $$unsubscribe_path();
  return `<aside class="top-0 left-0 w-64 h-screen fixed bg-background-200 dark:bg-darkbackground-500 svelte-7w3m9l"><ul class="flex flex-col overflow-hidden svelte-7w3m9l">${each(pages, (page) => {
    return `<li class="${[
      "hover:bg-primary-200 active:bg-primary-300 py-2 px-4 font-semibold svelte-7w3m9l",
      selected === page ? "bg-primary-400" : ""
    ].join(" ").trim()}">${escape(page.text)} </li>`;
  })}</ul> <hr class="svelte-7w3m9l"> <button class="text-light bg-primary-500 hover:bg-primary-400 font-medium rounded-lg text-sm px-5 py-2.5 text-center mx-4 mb-2 mt-4 dark:bg-primary-300 dark:hover:bg-primary-200  svelte-7w3m9l" data-svelte-h="svelte-wk9n2p">Toggle Dark Mode</button></aside> <main class="top-0 right-0 pl-64 mx-4 svelte-7w3m9l">${validate_component(selected.value || missing_component, "svelte:component").$$render($$result, {}, {}, {})}</main> `;
});
export {
  Page as default
};
