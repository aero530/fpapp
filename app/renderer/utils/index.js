/**
 * @function cumulativeSum
 * @description sort input object by key and calculate a cumulative sum.
 * @param {Object} inputObject object of numeric values to perform cumulative sum on.
 * @returns {Object} object of cumulative sum having the same keys as the original object.
 */
export function cumulativeSum(inputObject) {
  const output = {};
  let total = 0;
  Object.keys(inputObject).sort((a, b) => a - b).forEach((key) => {
    total += inputObject[key];
    output[key] = total;
  });
  return output;
}

/**
 * @function objectSubtract
 * @description piecewise subtraction of the values in two objects
 * @param {Object} a single layer object of numeric values (must have the same keys as b).
 * @param {Object} b single layer object of numeric values (must have the same keys as a).
 * @returns {Object} object with same keys are input objects and value of a[key]-b[key]
 */
export function objectSubtract(a, b) {
  const output = {};
  Object.keys(a).forEach((key) => {
    output[key] = a[key] - b[key];
  });
  return output;
}

/**
 * @function arraySum
 * @description calculate a sum of an array.
 * @param {Array} a array of numeric values to perform sum on.
 * @returns {number} sum of array values.
 */
export function arraySum(input) {
  let output = 0;
  input.forEach((value) => {
    output += value;
  });
  return output;
}

/**
 * @function formatDataObjects
 * @description format data objects for use in graphing
 * @param {Array} accounts array of data objects.
 * @returns {Array} array of objects to graphing.
 */
export function formatDataObjects(accounts) {
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
