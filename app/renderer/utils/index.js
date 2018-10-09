export function cumulativeSum(inputObject) {
  const output = {};
  let total = 0;
  Object.keys(inputObject).sort((a, b) => a - b).forEach((key) => {
    total += inputObject[key];
    output[key] = total;
  });
  return output;
}

export function cumulativeSumArray(a) {
  let result = a[0];
  for (let i = 1; i < a.length; i += 1) {
    result += a[i];
  }
  return result;
}

export function objectSubtract(a, b) {
  const output = {};
  Object.keys(a).forEach((key) => {
    output[key] = a[key] - b[key];
  });
  return output;
}

export function arraySum(input) {
  let output = 0;
  input.forEach((value) => {
    output += value;
  });
  return output;
}

export function formatDataObjects(accounts) {
  // dataIn is array of data objects
  const output = [];
  const years = Object.keys(accounts[0].data).sort((a, b) => a - b);

  years.forEach((year) => {
    let row = { x: year };
    accounts.forEach((account) => {
      row = { ...row, [account.name]: account.data[year] };
    });
    output.push(row);
  });
  return output;
}
