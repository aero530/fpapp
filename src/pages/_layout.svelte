<script>
    import {url} from '@roxi/routify'
	import "carbon-components-svelte/css/white.css";
	import {
        Header,
        SideNav,
        SideNavItems,
        SideNavMenu,
        SideNavMenuItem,
        SideNavLink,
        SideNavDivider,
        SkipToContent,
        Content,
        Grid,
        Row,
        Column,
	} from "carbon-components-svelte";
    
    import Dashboard16 from "carbon-icons-svelte/lib/Dashboard16";
    import Settings16 from "carbon-icons-svelte/lib/Settings16";
    import Money16 from "carbon-icons-svelte/lib/Money16";
    import PiggyBank16 from "carbon-icons-svelte/lib/PiggyBank16";
    import ShoppingCart16 from "carbon-icons-svelte/lib/ShoppingCart16";
    import Medication16 from "carbon-icons-svelte/lib/Medication16";
    import Education16 from "carbon-icons-svelte/lib/Education16";
    import Home16 from "carbon-icons-svelte/lib/Home16";
    import GroupSecurity16 from "carbon-icons-svelte/lib/GroupSecurity16";
    import Hourglass16 from "carbon-icons-svelte/lib/Hourglass16";

	let isSideNavOpen = false;
</script>

<Header company="" platformName="fpapp" bind:isSideNavOpen>
    <div slot="skip-to-content">
      <SkipToContent />
    </div>
</Header>
  
<SideNav bind:isOpen={isSideNavOpen} rail>
    <SideNavItems>
        <SideNavLink icon={Dashboard16} href={$url("./index")} text="Dashboard"/>
        <SideNavLink icon={Settings16} href={$url("./Settings")} text="Settings"/>
        <SideNavDivider />
        <SideNavLink icon={Education16} href={$url("./College")} text="College"/>
        <SideNavLink icon={ShoppingCart16} href={$url("./Expenses")} text="Expenses"/>
        <SideNavLink icon={Medication16} href={$url("./Hsa")} text="HSA"/>
        <SideNavLink icon={Money16} href={$url("./Income")} text="Income"/>
        <SideNavLink icon={Home16} href={$url("./Mortgage")} text="Mortgage"/>
        <SideNavLink icon={Hourglass16} href={$url("./Retirement")} text="Retirement"/>
        <SideNavLink icon={PiggyBank16} href={$url("./Savings")} text="Savings"/>
        <SideNavLink icon={GroupSecurity16} href={$url("./Ssa")} text="SSA"/>
      
        <SideNavMenu text="Menu">
            <SideNavMenuItem href="/" text="Link 1" />
            <SideNavMenuItem href="/" text="Link 2" />
            <SideNavMenuItem href="/" text="Link 3" />
          </SideNavMenu>
          <SideNavLink text="Link 4" />
    </SideNavItems>
</SideNav>
<main>
<Content>
    <Grid>
      <Row>
        <Column>
            <slot />
        </Column>
      </Row>
    </Grid>
</Content>

</main>


<style>
main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
}

@media (min-width: 640px) {
    main {
        max-width: none;
    }
}
</style>