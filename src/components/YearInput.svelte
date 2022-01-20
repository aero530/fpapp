<script>
    import {
        TextInput,
    } from "carbon-components-svelte";

    export let labelText;
    export let placeHolder;
    export let value;

    let invalid = false;
    let invalidText = "";
    
    function checkSuggestion(testValue) {
        let yearSuggestions = ["yearStart", "yearRetire", "yearDie", "yearEnd"];
        if (yearSuggestions.some(sug => testValue==sug)) {
            return true;
        } else {
            invalidText += "Unable to parse "+testValue+".  Valid inputs are yearStart, yearRetire, yearDie, & yearEnd. ";
            return false;
        }
    }

    function checkInt(testValue) {        
        if (isNaN(testValue)) {
            invalidText += "Unable to parse "+testValue+" as an integer. ";
            return false;
        } else {
            return true;
        }
    }

    function handleChange(event) {
        let search = event.target.value;
        let result;
        let isValid = false;
        invalidText = "";

        if (isNaN(Number(search))) { // input is not just a number
            const regex = /[+-]/g;
            const found = search.match(regex);
            if (found == null) {
                // isValid = false;
                // invalidText += "I don't know what you were trying to do.  Please try again. ";
                result = search;
                isValid = checkSuggestion(search);
            } else if (found.length == 1) { // there is a single operation character
                const re = /\s*(?:[+-]|$)\s*/;
                const searchList = search.split(re);
                let isNeg = search.includes("-");
                let int = parseInt(searchList[1]);
                result = {
                    base: searchList[0],
                    delta: isNeg ? -1*int : int,
                };
                isValid = checkSuggestion(searchList[0]) && checkInt(Number(searchList[1]));
            // } else if (found == 0) { // there are no operation charaters
            //     result = search;
            //     isValid = checkSuggestion(search);
            } else { // there is more than one operation character
                isValid = false;
                invalidText += "Only a single operation is allowed. ";
            }
        } else if (search == '') {
            isValid = false;
            invalidText += "Enter a year, use a variable (yearStart, yearRetire, yearDie, yearEnd), or variable with a pos or neg offset (yearStart+3 or yearEnd-4). ";
        } else { // the input is just a number and is parsed as an int
            result = parseInt(search);
            isValid = checkInt(result);
        }

        if (isValid) {
            bind:value = result;
            invalid =  false;
        } else {
            invalid = true;
        }
    }

    // Parse the object version of input back into a string for display
    function parseValue(input) {
        console.log(input);
        if (typeof input === 'object' && input !== null) {
            return input.base + (input.delta>0 ? "+"+input.delta.toString() : input.delta.toString());
        } else {
            return input
        }
    }

</script>

<TextInput
    labelText={labelText}
    placeHolder={placeHolder}
    value={parseValue(value)}
    on:change={handleChange}
    invalid={invalid}
    invalidText={invalidText}
/>