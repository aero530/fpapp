<script>
    
    import QuestionField from './QuestionField.svelte'

    export let label;
    export let value;
    export let questionText;

    let invalid = false;
    let helperText = "";
    
    function checkSuggestion(testValue) {
        let percentSuggestions = ["inflationBase"];
        if (percentSuggestions.some(sug => testValue==sug)) {
            return true;
        } else {
            helperText += "Unable to parse "+testValue+".  Valid input is inflationBase. ";
            return false;
        }
    }

    function checkFloat(testValue) {        
        if (isNaN(testValue)) {
            helperText += "Unable to parse "+testValue+" as a number. ";
            return false;
        } else if (testValue > 100 || testValue < 0) {
            helperText += "Value is out of bounds.  Must be between 0 and 100.";
            return false;
        } else {
            return true;
        }
    }

    function handleChange(event) {
        let search = event.target.value;
        let result;
        let isValid = false;
        helperText = "";

        
        console.log("value is " + search);

        if (isNaN(Number(search))) { // input is not just a number
            result = search;
            isValid = checkSuggestion(search);
        } else if (search == '') {
            isValid = false;
            helperText += "Enter a percentage or set to inflationBase. ";
        } else { // the input is just a number and is parsed as an int
            result = parseFloat(search);
            isValid = checkFloat(result);
        }

        if (isValid) {
            bind:value = result;
            invalid = false;
        } else {
            invalid = true;
        }
    }

    // Parse the object version of input back into a string for display
    function parseValue(input) {
        if (typeof input === 'object' && input !== null) {
            return input.base + (input.delta>0 ? "+"+input.delta.toString() : input.delta.toString());
        } else {
            return input
        }
    }

</script>


<QuestionField>
    <span slot="input">
        <input
            label={label}
            value={parseValue(value)}
            on:change={handleChange}
            invalid={invalid}
        />
            <!-- <HelperText persistent={invalid} slot="helper">{questionText + " " + helperText}</HelperText> -->

    </span>
    <span slot="questionTip">
        Percentage can be a number (such at 15) or inflationBase. 
    </span>
</QuestionField>
