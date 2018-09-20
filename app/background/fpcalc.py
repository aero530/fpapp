
# ======================================================================
# Notes
# ----------------------------------------------------------------------
# All data points are in that years evaluation of dollars (ie data for 2030 is in 2030 dollars)
# ======================================================================



# http://www.bankrate.com/calculators/retirement/retirement-plan-calculator.aspx



# ======================================================================
# Revision History
# ----------------------------------------------------------------------
# 9.1.12   - Initial development and numbers.
# 12.22.12 - Update numbers with Megan and set plan/budget for 2013.  
#          Reduced spending in retirement for retail and car.  Updated 
#          mortgate info with new loan amount.
# 10.27.13 - Update numbers
# http://money.msn.com/retirement/retirement-calculator.aspx
# 4-6% return is conservative for retirement planning
# to retire at 65 and live till 90 without social security we need $3.4M at retirement (to have 80% of current salary)
# 12.27.14 - Update numbers
# 3/10/2017 - Update numbers
# ======================================================================


# ======================================================================
# Time Inputs
# ----------------------------------------------------------------------
# age.now : Current age (in years) [30]
# age.retire : Age when you will start drawing from retirement accounts [65]
# age.die : Age when you will die (stop drawing from retirement accounts) [90]
# year_start : Current calendar year [2012]
# 
# ----------------------------------------------------------------------
# Time Computed Values
# ----------------------------------------------------------------------
# year_retire : Calendar year when you will start pulling from retirement accounts
# year_die : Calendar year when you will start pulling from retirement accounts
# year_delta : Table of how many years it has been since year_start
# year_table : Table of all years from year_start to year_die
# ======================================================================

age_retire = 67
age_die = 100
year_born = 1982
year_start = 2014 # 10/27/2013
year_start = 2015 # 12/27/2014
year_start = 2016 # 11/18/2015
year_start = 2017 # 12/31/2017

age_now = year_start - year_born
year_retire = year_born+age_retire
year_die = year_born+age_die
year_delta = range(age_now-age_now, age_die-age_now+1)
year_table = [x+year_start for x in year_delta]

# ======================================================================
# Inflation and Tax Inputs
# ----------------------------------------------------------------------
# inflation_base : Inflation rate as a percentage. (Used to increase the cost of living) [2.7]
# tax.income : Percent of income which goes to taxes [20.0]
# tax.capitalgains : Capital gains taxes (tax on interest earned) [15.0]
# inflation.retirementcostofliving : Cost of living during retirement relative to current cost of living as a percentage. [100.0]
#      Note that it is already accounted for that you will not be saving for retirement 
#      or college funds.  This is a blanket decrease in the amount of money spent on all 
#      expenses (including food, housing, medical, travel, etc.)
# ======================================================================

inflation_base = 3.0
tax_income = 20.0
tax_capitalgains = 15.0
retirement_costofliving = 85.0 # setting this to 100% will cover all of the expense category items which currently does

# ======================================================================
# Income Object
# ----------------------------------------------------------------------
# income(index).name : String describing this income source
# income(index).initial : Starting income
# income(index)['startin'] : Calendar year when this income starts
# income(index)['endin'] : Calendar year when this income ends (generally this is 
#      going to be the year before year_retire since the endin year is inclusive)
# income(index).increase : Percent increase in income every year
# ======================================================================



# ======================================================================
# Account Object
# ----------------------------------------------------------------------
# account(index)['type'] : Type of account
#      SAVINGS : Savings accounts should be used for any account where you 
#            accumulate wealth such as a bank savings account, money market account,
#            Roth IRA, or 401K.  Withdrawals from this account go into income
#            or net depending on what the tax status setting is.  Can have employer 
#            matching on these accounts
#      EXPENSE : Expense accounts account for daily expenses such as grocery, car 
#            insurance, clothes, travel, entertainment, etc.  Money to pay for these
#            things comes out of after tax income then out of net.
#      MORTGAGE : Home mortgage
#      LOAN : General loan type.  Can be used for things like car loans or school 
#            loans...anything with a balance due, interest rate, and consistant payments
#      COLLEGE : 529 account.  Withdrawals from this account will not go toward income or net
# account(index).name : String describing this income source
# account(index).table(1) : Starting balance.  For LOAN and MORTGAGE this should be a negative number (money is owed)
# account(index)['startin'] : Calendar year when money starts coming out of income and going into this account
# account(index)['endin'] : Calendar year when money no longer goes to this account (this is inclusive so it will generally be year_retire-1)
# account(index)['startout'] : Calendar year when money starts coming out of this account and acts as income
# account(index)['endout'] : Calendar year when money no longer is taken out of this account
# account(index)['yearlycontribution'] : Amount put into this accout every year
# account(index)['contribution']type : Type of contribution
#      fixed : fixed dollar amount
#      percent_of_income : percent of cost of current living
#      fixed_with_inflation : fixed dollar amount compensated for inflation from year 
#            start (ie dollar amount is in current dollars)
# account(index).yearlyreturn : Percent interest earned each year
# account(index)['withdrawaltype'] : How money should be removed from the account
#      end_at_zero : take money out in equal amounts each year such that the 
#            balance at endout is zero
#      fixed : Take out a fixed dollar amount
#      COL_fraction_of_total_savings : Take out the current cost of living * (this accounts value / total savings)
# account(index)['withdrawalvalue'] : How much money should be take out per year 
#      (either as a percentage or a fixed dollar amount)
# account(index)['paymenttype'] : How money should be removed from the account
#      fixed : fixed dollar amount
#      fixed_with_inflation : fixed dollar amount compensated for inflation from year 
#            start (ie dollar amount is in current dollars)
# account(index)['paymentvalue'] : How much money should be payed each year
#      (either as a percentage or a fixed dollar amount)
# account(index)['taxstatus'] : 
#      0=payed with taxed income, earnings are tax deferred, withdrawals are not taxed
#      1=payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
#            (tax free as long as used for intended purpose)
#      ## NOT IMPLIMENTED ## 2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
#      3=payed pretax and taxed in year of use as income
#      4=payed pretax and not taxed as income (use with HSA)
# account(index)['rate'] : Interest rate on borrowed money. This is an APR this is 
#      then compounded based on the compound time setting.  Used for LOAN and 
#      MORTGAGE account types.
# account(index)['compoundtime'] : Number of times per year that interest
#       is compounded. (1=yearly, 12=monthly) Used for MORTGAGE account types.
# account(index)['mortgageinsurance'] : Mortage insurance payment expressed as 
#      a yearly fixed number in today's dollars
# account(index)['ltvlimit'] : Loan to Value amount when mortage insurance is no 
#      longer pulled from payment.  Since monthly payment does not change over 
#      time, after the insurance is done there is more money going to the 
#      principal each payment
# account(index)['escrow'] : Amount of money going into escrow every year to pay 
#      for property tax.  This number is currently assumed to be constant 
#      (ie property taxes do not increase)
# account(index)['value'] : Current value of the home.  This is used to compute loan to value
# account(index)['raise'] : Yearly increase in income as a percent
# ======================================================================

INCOME = 1
RETIREMENT = 2
COLLEGE = 3
EXPENSE = 4
LOAN = 5
MORTGAGE = 6
SAVINGS = 7
HSA = 8

FIXED = 1
PERCENT_OF_INCOME = 2
FIXED_WITH_INFLATION = 3
END_AT_ZERO = 4
COL_FRAC_OF_SAVINGS = 5 # cost of living fraction of total savings




account = []



# ======================================================================
# Notes
# ======================================================================

# Make sure the retail number I put in is okay.  2010=12,400    2011=20,300    2012=3,600(first half of the year)
# Make sure the house number I put in is okay.  2010=1060.05    2011=9701.30    2012=12020.03(first half of the year)

# ======================================================================
# Main Program
#
# net_table[yearindex] : running sum of un allocated money after income tax (ie checking account)
# expensetotal.table(yearindex,accountindex) : initialize table for expenses
# incometotaltaxable['table'][yearindex] : total of all the running incomes
# ======================================================================

# Define counts used to run for loops
number_accounts = len(account) # number of accounts defined in the account object
number_years = len(year_delta) # number of years to run the simulation
		
# ----------------------------------------------------------------------
# Loop through accounts to determine what order they should be processed in
# ----------------------------------------------------------------------
accountorder_index = [0]*number_accounts # make accountorder_index an array of the correct size
accountorder_table = [INCOME,HSA,EXPENSE,MORTGAGE,LOAN,COLLEGE,RETIREMENT,SAVINGS] # define the order in which accounts should be processed
accountindex = 0
print('Length of account {0:d}'.format(number_accounts))
print('Length of accountordertable {0:d}'.format(len(accountorder_table)))
for typeindex in range(0,len(accountorder_table)) : # for each type of account in accountorder_table
    #print('index {0:d}, type = {1:d}'.format(typeindex,accountorder_table[typeindex]))
    for index in range(0,number_accounts) : # loop through all account objects
        if account[index]['type'] == accountorder_table[typeindex] : # if this account type matches the current accountorder_table type
            #print(' account type {0:d}, account index {1:d}'.format(account[index]['type'],index))
            accountorder_index[accountindex]=index # add the account index to the accountorder_table
            accountindex = accountindex + 1 # increment how far we are though the accountorder_table

# ----------------------------------------------------------------------
# initialize tables to the correct sizes
# ----------------------------------------------------------------------
expensetotal_title = [0]*number_accounts
expensetotal_table = np.zeros((number_years,number_accounts)) # initialize expense total table
savingstotal_table = np.zeros(number_years)
costofliving_table = np.zeros(number_years)
incometotaltaxable_table = np.zeros(number_years)
incometotal_table = np.zeros(number_years)
incometotalaftertax_table = np.zeros(number_years)
net_table = np.zeros(number_years)
yearlybudget_table = np.zeros(number_years) # yearly budget


for accountindex in range(0,number_accounts) : # loop through all account objects
    # ----------------------------------------------------------------------
    # Loop through accounts to initialize internal tables to correct size
    # ----------------------------------------------------------------------
    tmp = account[accountindex]['table'][0] # temporarily store the first year value input in account definition
    account[accountindex]['table'] = np.zeros(number_years) # make 'table' an array of the correct size
    account[accountindex]['table'][0] = tmp # replace initial value
    # ----------------------------------------------------------------------
    # Initialize the interest table in LOAN and MORTGAGE
    # ----------------------------------------------------------------------
    if account[accountindex]['type']==LOAN or account[accountindex]['type']==MORTGAGE :
        account[accountindex]['interest'] = np.zeros(number_years)
        account[accountindex]['escrow']   = np.zeros(number_years)
    # ----------------------------------------------------------------------
    # Initialize the earnings table in SAVINGS, COLLEGE, and RETIREMENT
    # ----------------------------------------------------------------------
    if account[accountindex]['type']==SAVINGS or account[accountindex]['type']==COLLEGE or account[accountindex]['type']==RETIREMENT  or account[accountindex]['type']==HSA : 
       account[accountindex]['earnings'] = np.zeros(number_years)
       account[accountindex]['withdrawal'] = np.zeros(number_years)
    # ----------------------------------------------------------------------
    # Initialize the contribution table
    # ----------------------------------------------------------------------
    if 'yearlycontribution' in account[accountindex] : 
        account[accountindex]['contribution'] = np.zeros(number_years)
    # ----------------------------------------------------------------------
    # Initialize the contribution table
    # ----------------------------------------------------------------------
    if 'paymenttype' in account[accountindex] : 
        account[accountindex]['payment'] = np.zeros(number_years)
    # ----------------------------------------------------------------------
    # Initialize and add all the account names to expense total
    # ----------------------------------------------------------------------
    if account[accountindex]['type']!=INCOME : # if this is not an income account type
        expensetotal_title[accountindex]=account[accountindex]['name'] # add the account title to the expense total table


# ----------------------------------------------------------------------
# Print header for table screen print of data
# ----------------------------------------------------------------------
print('{0}\t{1}\t{2}\t{3}\t{4}\t{5}\t{6}\n'.format("Year","Net","Income","Taxable Income","Income After Taxes","Expenses","Total Savings (retirement, college, etc)"))


# ----------------------------------------------------------------------
# Main loop to loop through each year
# ----------------------------------------------------------------------
for yearindex in range(0, number_years) : # loop through all years

    # ----------------------------------------------------------------------
    # Initialize this year
    # ----------------------------------------------------------------------
    if yearindex>0 :
        net_table[yearindex] = net_table[yearindex-1] # initialize this year as the value from last year
  
    year_current = year_table[yearindex] # set the current year to this year
    
    # ----------------------------------------------------------------------
    # Loop through accounts to make contributions and withdrawals
    # ---------------------------------------------------------------------
    for accountindex in accountorder_index :
    
        # ----------------------------------------------------------------------
        # Initialize temp variables to zero
        # ----------------------------------------------------------------------
        earnings = 0 # earnings is money that an account gains (ie interest for a savings account or retirement account.  for an income account earnings is the yearly income)
        interest = 0 # interest is money that must be payed off (ie for a loan or mortgage)
        contribution = 0 # contribution is money that goes from income to a savings type account (savings, college, retirement, etc)
        employermatch = 0 # set employermatch to zero
        payment = 0 # payment is money that must come out of income
        withdrawal = 0 # withdrawal is money that may be considered income (depending on account type)
        expense = 0
        
        # ----------------------------------------------------------------------
        # Initialize the value of the account for this year
        # ----------------------------------------------------------------------
        if account[accountindex]['type']==EXPENSE or account[accountindex]['type']==INCOME :# if this is an EXPENSE or INCOME account
            account[accountindex]['table'][yearindex] = 0 # previous years value does not carry over (ie not an account that carries a balance)
        else : # this account type should carry over the value from last year
            if yearindex > 0 :
                account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex-1]

        # ----------------------------------------------------------------------
        # Calculate earnings
        # ----------------------------------------------------------------------
        if account[accountindex]['type']==SAVINGS or account[accountindex]['type']==COLLEGE or account[accountindex]['type']==RETIREMENT or account[accountindex]['type']==HSA : # if this is a SAVINGS or COLLEGE etc account
            earnings = account[accountindex]['table'][yearindex]*account[accountindex]['yearlyreturn']/100 # calculate earnings from interest
            account[accountindex]['earnings'][yearindex] = earnings # set account earnings to current earnings value
        elif account[accountindex]['type']==INCOME : # Otherwise if this is an INCOME account (here ear)
            if (account[accountindex]['startin']<=year_current) and (account[accountindex]['endin']>=year_current) : # if this income object is active this year 
                earnings = account[accountindex]['base'] * math.pow( (1+account[accountindex]['raise']/100) , (year_current-account[accountindex]['startin']) )# calculate this years income

        # ----------------------------------------------------------------------
        # Calculate interest
        # ----------------------------------------------------------------------
        if account[accountindex]['type']==LOAN : # otherwise if this is a LOAN account
            interest = account[accountindex]['table'][yearindex]*account[accountindex]['rate']/100.0
            account[accountindex]['interest'][yearindex] = interest
        elif account[accountindex]['type']==MORTGAGE : # Otherwise if this is a MORTGAGE account
            if (account[accountindex]['table'][yearindex]*100.0/account[accountindex]['value']>account[accountindex]['ltvlimit']) : # if the current loan to value is more than the cutoff limit
                #account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex]*(1+(account[accountindex]['rate']/100)/account[accountindex]['compoundtime'])^account[accountindex]['compoundtime'] - account[accountindex]['mortgageinsurance'] - account[accountindex]['escrow'] # add this years interest to the mortgage then decrease mortgage by payment amount but reduce payment by the escrow and mortgage insurance values
                interest = account[accountindex]['table'][yearindex]*math.pow( 1+((account[accountindex]['rate']/100.0)/account[accountindex]['compoundtime']),account[accountindex]['compoundtime']) + account[accountindex]['mortgageinsurance'] + account[accountindex]['escrowvalue'] - account[accountindex]['table'][yearindex]# add this years interest to the mortgage then decrease mortgage by payment amount but reduce payment by the escrow and mortgage insurance values
            else : # otherwise if the current loan to value is less than the cutoff limit
                #account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex]*(1+(account[accountindex]['rate']/100)/account[accountindex]['compoundtime'])^account[accountindex]['compoundtime'] - account[accountindex]['escrow'] # add this years interest to the mortgage athe make payment on mortgage but but reduce payment by escrow value
                interest = account[accountindex]['table'][yearindex]*math.pow( 1+((account[accountindex]['rate']/100.0)/account[accountindex]['compoundtime']),account[accountindex]['compoundtime']) + account[accountindex]['escrowvalue'] - account[accountindex]['table'][yearindex]# add this years interest to the mortgage athe make payment on mortgage but but reduce payment by escrow value
                account[accountindex]['interest'][yearindex] = interest
                account[accountindex]['escrow'][yearindex] = account[accountindex]['escrowvalue']   
            
        # ----------------------------------------------------------------------
        # Calculate contribution amount
        # ----------------------------------------------------------------------
        if 'yearlycontribution' in account[accountindex] : # if there contribution values are defined
		#if account[accountindex]['yearlycontribution']>0 : # if there contribution values are defined

            if (account[accountindex]['startin']<=year_current) and (account[accountindex]['endin']>=year_current) : # if making contribution this year
                # ----------------------------------------------------------------------
                # Calculate contribution amount based on contribution type
                # ----------------------------------------------------------------------
                if account[accountindex]['contributiontype']==FIXED_WITH_INFLATION : # if inflation needs to be accounted for in the contribution
                    contribution = account[accountindex]['yearlycontribution'] * math.pow((1+inflation_base/100),year_delta[yearindex]) # increase the value by inflation
                elif account[accountindex]['contributiontype']==FIXED : # otherwise if the contribution is a fixed value
                    contribution = account[accountindex]['yearlycontribution'] # set the contribution amount to the value input
                elif account[accountindex]['contributiontype']==PERCENT_OF_INCOME : # otherwise if the contribution is a percent of income
                    if 'incomelink' in account[accountindex] and account[accountindex]['incomelink']>=0 : # and if the account has an income object linked to it
                        contribution = account[account[accountindex]['incomelink']]['table'][yearindex] * (account[accountindex]['yearlycontribution']/100) # calculate the contribution value using that income account and the percentage input
                    else : # otherwise
                        contribution = incometotaltaxable_table[yearindex] * (account[accountindex]['yearlycontribution']/100) # calculate the contribution using the total income for the year
                
                account[accountindex]['contribution'][yearindex] = contribution # set account contribution value to current contribution value

                if contribution<0 :
                    print('Error contribution < 0')

                # ----------------------------------------------------------------------
                # Calculate the employer contribution
                # ----------------------------------------------------------------------
                if 'incomelink' in account[accountindex] and 'employermatch' in account[accountindex] and 'matchlimit' in account[accountindex] :
                    if  account[accountindex]['incomelink']>=0 : # if there is an incomelink for this account
                        if isinstance(account[accountindex]['employermatch'],float) or isinstance(account[accountindex]['employermatch'],int) : # if employermatch is not a list
                            tmp = account[accountindex]['employermatch']
                            account[accountindex]['employermatch'] = [tmp]
                        if isinstance(account[accountindex]['matchlimit'],float) or isinstance(account[accountindex]['matchlimit'],int) :
                            tmp = account[accountindex]['matchlimit']
                            account[accountindex]['matchlimit'] = [tmp]

                        if account[accountindex]['employermatch'][0]>=0 and account[accountindex]['matchlimit'][0]>=0 :
                            if len(account[accountindex]['matchlimit']) > 1 : # and if it is a complex employer matching (more than one level)
                                if contribution>=(account[accountindex]['matchlimit'][0]/100+account[accountindex]['matchlimit'][1]/100)*account[account[accountindex]['incomelink']]['table'][yearindex] : # and if the contribution is above the highest employer matching level
                                    employermatch = account[account[accountindex]['incomelink']]['table'][yearindex] * (  (account[accountindex]['employermatch'][1]/100)*(account[accountindex]['matchlimit'][1]/100)  +  (account[accountindex]['employermatch'][0]/100)*(account[accountindex]['matchlimit'][0]/100)  ) # calculate the employer matching based on the match limits
                                elif contribution>=account[accountindex]['matchlimit'][0]/100*account[account[accountindex]['incomelink']]['table'][yearindex] : # otherwise if the contribution is between the employer matching levels 
                                    employermatch = account[account[accountindex]['incomelink']]['table'][yearindex] * (  (account[accountindex]['employermatch'][0]/100)*(account[accountindex]['matchlimit'][0]/100)  +  (account[accountindex]['employermatch'][1]/100)*(account[accountindex]['matchlimit'][1]/100) * (contribution/account(account[accountindex]['incomelink'])['table'][yearindex]-account[accountindex]['matchlimit'][0]/100)  ) # calculate the employer matching with all the first level and part of the second level
                                else : # otherwise if below the first employer match limit
                                    employermatch = (contribution) * (account[accountindex]['employermatch'][0]/100) # the employer contribution is computed based on the entire contribution
                            else : # if it is a simple employer matching (only one level)
                                if contribution>=account[accountindex]['matchlimit'][0]*account[account[accountindex]['incomelink']]['table'][yearindex] : # and if the contribution is above the highest employer matching level
                                    employermatch = account[account[accountindex]['incomelink']]['table'][yearindex] * (account[accountindex]['employermatch'][0]/100)*(account[accountindex]['matchlimit'][0]/100) # calculate the employer matching based on the match limits
                                else : # otherwise  if below the employer match limit
                                    employermatch = (contribution) * (account[accountindex]['employermatch'][0]/100) # the employer contribution is computed based on the entire contribution
                    else :
                        print('Employer Match defined for account {} but incomelink<0'.format(account[accountindex]['name']))
                elif 'employercontribution' in account[accountindex] and (year_current <= year_retire) :
                    if account[accountindex]['contributiontype']==FIXED_WITH_INFLATION : # if inflation needs to be accounted for in the contribution
                        employermatch = account[accountindex]['employercontribution'] * math.pow((1+inflation_base/100),year_delta[yearindex]) # increase the value by inflation
                    elif account[accountindex]['contributiontype']==FIXED : # otherwise if the contribution is a fixed value
                        employermatch = account[accountindex]['employercontribution'] # set the contribution amount to the value input
                    else :
                        print('Employer Contribution type not implimented')

                             
        # ----------------------------------------------------------------------
        # Calculate payment
        # ----------------------------------------------------------------------
        if 'paymenttype' in account[accountindex] : # if there is a payment defined
            if (account[accountindex]['startout']<=year_current) and (account[accountindex]['endout']>=year_current) : # if making a payment this year 
            # ----------------------------------------------------------------------
            # Calculate payment amount
            # ----------------------------------------------------------------------_
                if account[accountindex]['paymenttype']==FIXED : # otherwise if type is a fixed value
                    payment = account[accountindex]['paymentvalue'] # set withdrawal to the value
                elif account[accountindex]['paymenttype']==FIXED_WITH_INFLATION : #otherwise if type is a fixed number but should be compensated for with inflation
                    payment = account[accountindex]['paymentvalue'] * (1+inflation_base/100)^(year_current-account[accountindex]['startout']) # set withdrawal to the value multiplied by an increase due to inflation
                else : # otherwise if a different type is specified
                    payment = 0 # set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)
                if payment > account[accountindex]['table'][yearindex] :
                    payment = account[accountindex]['table'][yearindex]
                account[accountindex]['payment'][yearindex] = payment #add payment to payment table
            if payment < 0 :
                print('Error payment < 0')


        # ----------------------------------------------------------------------
        # Calculate withdrawal
        # ----------------------------------------------------------------------
        if 'withdrawaltype' in account[accountindex] :
            if (account[accountindex]['startout']<=year_current) and (account[accountindex]['endout']>=year_current) : # if taking money out this year 
            
                # ----------------------------------------------------------------------
                # Calculate withdrawal amount
                # ----------------------------------------------------------------------_
                if account[accountindex]['withdrawaltype']==COL_FRAC_OF_SAVINGS : # otherwise if type is cost of living fraction of total savings
                    if (yearindex>1) :
                        #withdrawal = costofliving['table'][yearindex] * account[accountindex].table(yearindex-1)./savingstotal.table(yearindex-1)
                        # account for retirement cost of living and for capital gains in this line...its a hack and probably not very correct
                        if account[accountindex]['table'][yearindex-1] > 0 : # if there is money left in the account (python gives error on zero / anything)
                            # withdrawal from this account = total expenses this year  * fraction of total savings this account represents
                            withdrawal = expensetotal_table.sum(axis=1)[yearindex] * account[accountindex]['table'][yearindex-1]/savingstotal_table[yearindex-1]
                            if 'taxstatus' in account[accountindex] and (account[accountindex]['taxstatus']==3) :
                                withdrawal = withdrawal*(tax_income/100+1) # add extra to amount withdrawal value to account for taxes.
#                            print('{0:f}\t{1:f}\t{2:f}\t{3:f}'.format(expensetotal_table.sum(axis=1)[yearindex],withdrawal,account[accountindex]['table'][yearindex-1],savingstotal_table[yearindex-1]))
                    else :
                        print('ERROR - Can not compute withdrawal amount')

                elif account[accountindex]['withdrawaltype']==FIXED : # otherwise if type is a fixed value
                    withdrawal = account[accountindex]['withdrawalvalue'] # set withdrawal to the value
                elif account[accountindex]['withdrawaltype']==FIXED_WITH_INFLATION : #otherwise if type is a fixed number but should be compensated for with inflation
                    withdrawal = account[accountindex]['withdrawalvalue'] * math.pow((1+inflation_base/100),year_delta[yearindex]) # set withdrawal to the value multiplied by an increase due to inflation
                elif account[accountindex]['withdrawaltype']==END_AT_ZERO : # otherwise if the type is end at zero
                    if (account[accountindex]['endout']>=year_current) : # and if the year to stop taking money out of the account is beyond or equal to the current year
                        withdrawal = account[accountindex]['table'][yearindex]/(account[accountindex]['endout']-year_current+1) # calculate the fraction of the account balance to withdraw
                else : # otherwise if a different type is specified
                    print('Invalid withdrawal type')
                    withdrawal = 0 # set withdrawal to zero (this is for accounts that you dont remove money from such as expense accounts)

            if withdrawal > account[accountindex]['table'][yearindex] : #dont allow an account to become overdrawn
                withdrawal = account[accountindex]['table'][yearindex]
            
            if withdrawal < 0 :
                print('Error withdrawal < 0')

            account[accountindex]['withdrawal'][yearindex] = withdrawal   


        # ----------------------------------------------------------------------
        # Calculate expense amount
        # ----------------------------------------------------------------------
        if 'expensetype' in account[accountindex] : # if there is a expense type defined
            if (account[accountindex]['startout']<=year_current) and (account[accountindex]['endout']>=year_current) : # if this expense applies this year
            
                # ----------------------------------------------------------------------
                # Calculate expense amount
                # ----------------------------------------------------------------------_
                if account[accountindex]['expensetype']==FIXED : # otherwise if type is a fixed value
                    expense = account[accountindex]['expensevalue'] # set expense to the value
                elif account[accountindex]['expensetype']==FIXED_WITH_INFLATION : #otherwise if type is a fixed number but should be compensated for with inflation
                    expense = account[accountindex]['expensevalue'] * math.pow(1+inflation_base/100,year_delta[yearindex]) # set expense to the value multiplied by an increase due to inflation
                else : # otherwise if a different type is specified
                    expense = 0 # set expense to zero (this is for accounts that you dont remove money from such as expense accounts)
                    print('Invalid expense type')
                if (year_current >= year_retire) :
                    expense = expense * (retirement_costofliving/100)
            if expense < 0 :
                print('Error expense < 0')


# #      0=payed with taxed income, earnings are tax deferred, withdrawals are not taxed
# #      1=payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
# #            (tax free as long as used for intended purpose)
# #      2=payed with taxed income, earnings are taxed in year taken out as capital gains, withdrawals are not taxed
# #      3=payed pretax and taxed in year of use as income

        # ----------------------------------------------------------------------
        # Add earnings to the account for the year
        # ----------------------------------------------------------------------
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] + earnings
        
        # ----------------------------------------------------------------------
        # Add earnings to income tables for the year
        # ----------------------------------------------------------------------        
        if account[accountindex]['type']==INCOME :
            incometotaltaxable_table[yearindex] = incometotaltaxable_table[yearindex] + account[accountindex]['table'][yearindex]  # increase this years taxable income by the withdrawal amount
            incometotal_table[yearindex] = incometotal_table[yearindex] + account[accountindex]['table'][yearindex]  # increase this years income by the withdrawal amount
        
        # ----------------------------------------------------------------------
        # Add capital gains earnings to taxable income
        # ----------------------------------------------------------------------
#        if 'taxstatus' in account[accountindex] and account[accountindex]['taxstatus']==1 : # payed with taxed income, earnings are taxed in year earned as capital gains, withdrawals are not taxed
#            incometotaltaxable_table[yearindex] = incometotaltaxable_table[yearindex] + earnings * tax_capitalgains/100  # increase this years taxable income by the capital gains for this account
         
        # ----------------------------------------------------------------------
        # Add interest to the account for the year
        # ----------------------------------------------------------------------
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] + interest

        # ----------------------------------------------------------------------
        # Add contribution and employermatch to the account for the year
        # ----------------------------------------------------------------------
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] + contribution + employermatch
        if 'taxstatus' in account[accountindex] and (account[accountindex]['taxstatus']==3 or account[accountindex]['taxstatus']==4) : # if contributions should be taken out of taxable income for the year
            incometotaltaxable_table[yearindex] = incometotaltaxable_table[yearindex] - contribution # take the contribution value out of taxable income for the year 


        # ----------------------------------------------------------------------
        # Add payment to the account for the year
        # ----------------------------------------------------------------------
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] - payment

        # ----------------------------------------------------------------------
        # Add withdrawal to the account for the year
        # ----------------------------------------------------------------------
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] - withdrawal
        if account[accountindex]['type'] != COLLEGE : # dont put college withdrawals into income because they go to kids not me
            if 'taxstatus' in account[accountindex] and account[accountindex]['taxstatus']==3 : # if the withdrawal should be counted as taxable income for the year
                incometotaltaxable_table[yearindex] = incometotaltaxable_table[yearindex] + withdrawal  # increase this years taxable income by the withdrawal amount
                incometotal_table[yearindex] = incometotal_table[yearindex] + withdrawal  # increase this years taxable income by the withdrawal amount
            else :
                incometotal_table[yearindex] = incometotal_table[yearindex] + withdrawal  # increase this years taxable income by the withdrawal amount

        # ----------------------------------------------------------------------
        # Remove earnings value from net in year taken out
        # earnings are taxed in year taken out as capital gains, withdrawals are not taxed
        # ----------------------------------------------------------------------
        #if 'taxstatus' in account[accountindex] and account[accountindex]['taxstatus']==2 :
        #    net_table[yearindex] = net_table[yearindex] - sum(account[accountindex]['earnings']) / sum(account[accountindex]['contribution']) * withdrawal * tax_capitalgains/100
        #    yearlybudget_table[yearindex] = yearlybudget_table[yearindex] - sum(account[accountindex]['earnings']) / sum(account[accountindex]['contribution']) * withdrawal * tax_capitalgains/100

        # ----------------------------------------------------------------------
        # Add expense to the account for the year
        # ----------------------------------------------------------------------
        if 'taxstatus' in account[accountindex] and (account[accountindex]['taxstatus']==3 or account[accountindex]['taxstatus']==4) : # if contributions should be taken out of taxable income for the year
        # this is really just paying for health insurance as it is the only expense that has a taxstatus = 3 or 4
            incometotaltaxable_table[yearindex] = incometotaltaxable_table[yearindex] - expense # take the expense value out of taxable income for the year                 
        
        account[accountindex]['table'][yearindex] = account[accountindex]['table'][yearindex] + expense
        
        if 'ishealthcare' in account[accountindex] and account[accountindex]['ishealthcare']==1 :
            #pull from HSA
            if 'hsalink' in account[accountindex] : #if there is an HSA account linked
                # print('{0:s} {1:.2f} {2:.2f}'.format(account[account[accountindex]['hsalink']]['name'],account[account[accountindex]['hsalink']]['table'][yearindex], expense))
                if account[account[accountindex]['hsalink']]['table'][yearindex] >= expense: # if there is enough money in the HSA savings account to pay for healthcare expenses
                    account[account[accountindex]['hsalink']]['table'][yearindex] = account[account[accountindex]['hsalink']]['table'][yearindex] - expense
                else : # otherwise drain the HSA account then reset expense to represent the remaining balance of the expense
                    tmp = expense - account[account[accountindex]['hsalink']]['table'][yearindex]
                    account[account[accountindex]['hsalink']]['table'][yearindex] = 0
                    expense = tmp


        # ----------------------------------------------------------------------
        # Add entry to expense total table
        # ----------------------------------------------------------------------
        if account[accountindex]['type']==EXPENSE : # and if type is EXPENSE
            expensetotal_table[yearindex][accountindex]=expense # add withdrawal to the expense table
        elif account[accountindex]['type']==LOAN : # otherwise if type is LOAN
            expensetotal_table[yearindex][accountindex]=payment # add withdrawal to the expense table
        elif account[accountindex]['type']==MORTGAGE : # otherwise if type is LOAN
            expensetotal_table[yearindex][accountindex]=payment # add withdrawal to the expense table
        elif account[accountindex]['type']==COLLEGE :  # otherwise if type is LOAN
            expensetotal_table[yearindex][accountindex]=contribution # add contribution to the expense table
        elif account[accountindex]['type']==SAVINGS : # otherwise if type is a SAVINGS account
            expensetotal_table[yearindex][accountindex]=contribution # add contribution to the expense table
        elif account[accountindex]['type']==RETIREMENT : # otherwise if type is a SAVINGS account
            expensetotal_table[yearindex][accountindex]=contribution # add contribution to the expense table
        elif account[accountindex]['type']==HSA : # otherwise if type is a HSA account
            expensetotal_table[yearindex][accountindex]=contribution # add contribution to the expense table


        # ----------------------------------------------------------------------
        # Add entry to savings total
        # ----------------------------------------------------------------------
        if account[accountindex]['type']==SAVINGS or account[accountindex]['type']==RETIREMENT : # if this is a savings account
            savingstotal_table[yearindex] = savingstotal_table[yearindex] + account[accountindex]['table'][yearindex]
		
        # print('{0} {1:.2f} {2:.2f} {3:.2f} {4:.2f} {5:.2f} {6:.2f} {7:.2f}'.format(account[accountindex]['name'], earnings, interest, contribution, employermatch, payment, withdrawal, expense))


    # ----------------------------------------------------------------------
    # Add Income to net account (subtract out paying for income tax)
    # ----------------------------------------------------------------------
    net_table[yearindex] = net_table[yearindex] + incometotal_table[yearindex] - incometotaltaxable_table[yearindex]*(tax_income/100) - expensetotal_table.sum(axis=1)[yearindex]
    incometotalaftertax_table[yearindex] = incometotal_table[yearindex] - incometotaltaxable_table[yearindex]*(tax_income/100)

    print('{0:d}\t{1:9.2f}\t{2:9.2f}\t{3:9.2f}\t{4:9.2f}\t{5:9.2f}\t{6:9.2f}'.format(year_current, net_table[yearindex], incometotal_table[yearindex], incometotaltaxable_table[yearindex], incometotalaftertax_table[yearindex], expensetotal_table.sum(axis=1)[yearindex], savingstotal_table[yearindex] ))


# ======================================================================
# Generate Graphs
# ======================================================================

# http://colorbrewer2.org/


# ======================================================================
# Define color list for matplotlib plots
# ======================================================================

# Qualatative Colors
#color_list = [(0.651,0.808,0.89), (0.122,0.471,0.706), (0.698,0.875,0.541), (0.2,0.51,0.173), (0.984,0.604,0.6), (0.89,0.102,0.11), (0.992,0.749,0.435), (1,0.498,0), (0.792,0.698,0.839), (0.416,0.239,0.604), (1,1,0.6)]
color_list = [(0.651,0.808,0.89), (0.122,0.471,0.706), (0.698,0.875,0.541), (0.2,0.51,0.173), (0.984,0.604,0.6), (0.89,0.102,0.11), (0.992,0.749,0.435), (1,0.498,0), (0.792,0.698,0.839), (0.416,0.239,0.604), (1,1,0.6), (0.62,0.004,0.259), (0.835,0.243,0.31), (0.957,0.427,0.263), (0.992,0.682,0.38), (0.996,0.878,0.545), (1,1,0.749), (0.902,0.961,0.596), (0.671,0.867,0.643), (0.4,0.761,0.647), (0.196,0.533,0.741), (0.369,0.31,0.635)]
# Diverging Colors
#color_list = [(0.62,0.004,0.259), (0.835,0.243,0.31), (0.957,0.427,0.263), (0.992,0.682,0.38), (0.996,0.878,0.545), (1,1,0.749), (0.902,0.961,0.596), (0.671,0.867,0.643), (0.4,0.761,0.647), (0.196,0.533,0.741), (0.369,0.31,0.635)]


# ----------------------------------------------------------------------
# Close currently open
# ----------------------------------------------------------------------
#close open figure windows (up to 101 of them)
for index in range(0,100) :
    plt.close()


# list of figures
figures = [];
axis = [];
legends = [];

figures.append(plt.figure())
axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
axis[-1].plot(year_delta, net_table, linewidth=2, label='Net')
axis[-1].plot(year_delta, incometotal_table, linewidth=2, label='Total Income')
axis[-1].plot(year_delta, incometotal_table-incometotaltaxable_table*(tax_income/100), linewidth=2, label='After Tax Income')
axis[-1].plot(year_delta, expensetotal_table.sum(axis=1), linewidth=2, label='Expenses')
legends.append(axis[-1].legend(loc='best', fancybox=True))
legends[-1].get_frame().set_alpha(0.5)

# ----------------------------------------------------------------------
# Generate HSA graph
# ----------------------------------------------------------------------
for account_current in account :
    if account_current['type']==HSA :
        proxy_artist_name = []
        proxy_artist_object = []     
        figures.append(plt.figure())
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
        
        axis[-1].plot(year_delta, account_current['table'], linewidth=2, label='Account Value')
        axis[-1].plot(year_delta, account_current['contribution'], linewidth=2, label='Contribution')
        axis[-1].plot(year_delta, account_current['earnings'], linewidth=2, label='Earnings')

        axis[-1].set_title(account_current['name'])
        axis[-1].set_xlabel("Year Index")
        axis[-1].set_ylabel("Dollars")
        legends.append(axis[-1].legend(loc='upper right', fancybox=True))
        legends[-1].get_frame().set_alpha(0.5)


# ----------------------------------------------------------------------
# Generate retirement account graph
# ----------------------------------------------------------------------
retirement_plot = []
proxy_artist_name = []
proxy_artist_object = []
index = 0
for account_current in account :
    if account_current['type']==RETIREMENT :
        retirement_plot.append(account_current['table'])
        proxy_artist_name.append(account_current['name'])
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[index]))
        if index < len(color_list)-1 :
            index = index + 1
        else :
            index = 0
figures.append(plt.figure())
axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
axis[-1].stackplot(year_delta, retirement_plot, colors=color_list)
legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper left', fancybox=True))
legends[-1].get_frame().set_alpha(0.5)
axis[-1].set_title('Retirement Accounts')

retirement_chart = pygal.StackedBar(legend_at_bottom=True, legend_at_bottom_columns=4, style=my_style, x_label_rotation=45)
retirement_chart.title = 'Retirement Accounts'
retirement_chart.x_labels = map(str, year_table)
for account_current in account :
    if account_current['type']==RETIREMENT :
        retirement_chart.add(account_current['name'], account_current['table'])
retirement_chart.value_formatter = lambda x: "%.0f" % x
retirement_chart.render_to_file('retirement.svg')

# ----------------------------------------------------------------------
# Generate graphs for Loan graphs
# ----------------------------------------------------------------------
for account_current in account :
    if account_current['type']==LOAN :
        proxy_artist_name = []
        proxy_artist_object = []        
        figures.append(plt.figure())
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot

        proxy_artist_name.append("Cumulative Payment")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[0]))
        axis[-1].stackplot(year_delta, np.multiply(np.cumsum(account_current['payment']),np.ma.not_equal(account_current['payment'],0).data), colors=color_list)

        proxy_artist_name.append("Loan Value")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[1]))
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
        axis[-1].plot(year_delta, account_current['table'])
        
        proxy_artist_name.append("Yearly Payment")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[2]))        
        axis[-1].plot(year_delta, account_current['payment'])

        axis[-1].set_title(account_current['name'])
        axis[-1].set_xlabel("Year Index")
        axis[-1].set_ylabel("Dollars")
        legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper right', fancybox=True))
        legends[-1].get_frame().set_alpha(0.5)


# ----------------------------------------------------------------------
# Generate mortgage graph
# ----------------------------------------------------------------------

for account_current in account :
    if account_current['type']==MORTGAGE :
        proxy_artist_name = []
        proxy_artist_object = []        
        figures.append(plt.figure())
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot

        proxy_artist_name.append("Cumulative Payment")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[0]))
        axis[-1].stackplot(year_delta, np.multiply(np.cumsum(account_current['payment']) - np.cumsum(account_current['escrow']),np.ma.not_equal(account_current['table'],0).data), colors=color_list)

        proxy_artist_name.append("Mortgage Value")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[1]))
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
        axis[-1].plot(year_delta, account_current['table'])
        
        proxy_artist_name.append("Yearly Payment")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[2]))        
        axis[-1].plot(year_delta, account_current['payment'])
        
        axis[-1].set_title(account_current['name'])
        axis[-1].set_xlabel("Year Index")
        axis[-1].set_ylabel("Dollars")
        legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper right', fancybox=True))
        legends[-1].get_frame().set_alpha(0.5)


# ----------------------------------------------------------------------
# Generate college graphs
# ----------------------------------------------------------------------

for account_current in account :
    if account_current['type']==COLLEGE :
        proxy_artist_name = []
        proxy_artist_object = []        
        figures.append(plt.figure())
        axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot

        proxy_artist_name.append("Start")
        proxy_artist_name.append("Contribution")
        proxy_artist_name.append("Earnings")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[0]))
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[1]))
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[2]))
        
#        axis[-1].stackplot(year_delta, np.multiply(np.ones_like(account_current['contribution']),account_current['table'][0]-account_current['contribution'][0]-account_current['earnings'][0]), np.multiply(np.cumsum(account_current['contribution']),np.ma.not_equal(account_current['contribution'],0).data), np.multiply(np.cumsum(account_current['earnings']),np.ma.not_equal(account_current['earnings'],0).data), colors=color_list)
        
        start = np.multiply(np.multiply(np.ones_like(account_current['contribution']),account_current['table'][0]-account_current['contribution'][0]-account_current['earnings'][0]),np.ma.not_equal(account_current['contribution'],0).data)
        contribution = start + np.multiply(np.cumsum(account_current['contribution']),np.ma.not_equal(account_current['contribution'],0).data)  
        earnings = contribution + np.multiply(np.cumsum(account_current['earnings']),np.ma.not_equal(account_current['contribution'],0).data)

        axis[-1].fill_between(year_delta, 0, earnings, facecolor=color_list[2])
        axis[-1].fill_between(year_delta, 0, contribution, facecolor=color_list[1])
        axis[-1].fill_between(year_delta, 0, start, facecolor=color_list[0])
        
        proxy_artist_name.append("Account Value")
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[3]))
        axis[-1].plot(year_delta, account_current['table'])
        axis[-1].set_title(account_current['name'])
        axis[-1].set_xlabel("Year Index")
        axis[-1].set_ylabel("Dollars")
        legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper right', fancybox=True))
        legends[-1].get_frame().set_alpha(0.5)

# ----------------------------------------------------------------------
# Generate expenses graph
# ----------------------------------------------------------------------
expenses_plot = []
proxy_artist_name = []
proxy_artist_object = []
index = 0
for account_current in account :
    if account_current['type']==EXPENSE or account_current['type']==LOAN or account_current['type']==MORTGAGE or account_current['type']==SAVINGS or account_current['type']==COLLEGE or account_current['type']==RETIREMENT :
        if account_current['type']==EXPENSE:
            expenses_plot.append(account_current['table'])
        if account_current['type']==LOAN or account_current['type']==MORTGAGE :
            expenses_plot.append(account_current['payment'])
        if account_current['type']==COLLEGE or account_current['type']==SAVINGS or account_current['type']==RETIREMENT :
            expenses_plot.append(account_current['contribution'])
        proxy_artist_name.append(account_current['name'])
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[index]))
        if index < len(color_list)-1 :
            index = index + 1
        else :
            index = 0
figures.append(plt.figure())
axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
axis[-1].stackplot(year_delta, expenses_plot, colors=color_list)
legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper left', fancybox=True))
legends[-1].get_frame().set_alpha(0.5)


# ----------------------------------------------------------------------
# Generate income graph
# ----------------------------------------------------------------------
income_plot = []
proxy_artist_name = []
proxy_artist_object = []
index = 0
for account_current in account :
    if account_current['type']==INCOME :
        income_plot.append(account_current['table'])   
        proxy_artist_name.append(account_current['name'])
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[index]))
    elif account_current['type']==RETIREMENT :
        income_plot.append(account_current['withdrawal'])
        proxy_artist_name.append(account_current['name'])
        proxy_artist_object.append(plt.Rectangle((0, 0), 1, 1, fc=color_list[index]))
    if index < len(color_list)-1 :
        index = index + 1
    else :
        index = 0

figures.append(plt.figure())
axis.append(figures[-1].add_subplot(111)) # 1 row, 1 col, first subplot
axis[-1].stackplot(year_delta, income_plot, colors=color_list)
legends.append(axis[-1].legend(proxy_artist_object,proxy_artist_name,loc='upper left', fancybox=True))
legends[-1].get_frame().set_alpha(0.5)

#line_chart = pygal.StackedLine(fill=True)
income_chart = pygal.StackedBar(legend_at_bottom=True, legend_at_bottom_columns=4, style=my_style, x_label_rotation=45)
income_chart.title = 'Income - Pretax'
income_chart.x_labels = map(str, year_table)
for account_current in account :
    if account_current['type']==INCOME :
        income_chart.add(account_current['name'], account_current['table'])
    elif account_current['type']==RETIREMENT :
        income_chart.add(account_current['name'], account_current['withdrawal'])
income_chart.value_formatter = lambda x: "%.0f" % x
income_chart.render_to_file('income.svg')