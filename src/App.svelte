<script lang="ts">
	import {
		Button,
		Card,
		CardBody,
		CardFooter,
		CardHeader,
		CardSubtitle,
		CardText,
		CardTitle,
		Container,
		Input,
	} from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	let input: string = '8';
	let result: string = '';
	const handleClick = async () => {
		result = await invoke('generate_password', {
			length: +input,
		});
	};
</script>

<main>
	<Container>
		<Card class="mb-3">
			<CardHeader>
				<CardTitle>Password Generator</CardTitle>
			</CardHeader>
			<CardBody>
				<CardSubtitle>Generate random password</CardSubtitle>
				<CardText>Press 'Generate' button to create password with length: {input}.</CardText
				>
				<Input
					type="range"
					id="exampleRange"
					bind:value={input}
					min={1}
					max={32}
					step={1}
				/>
				<Button color="primary" on:click={handleClick}>Generate</Button>
			</CardBody>
			<CardFooter>
				{#if result.length !== 0}
					{result}
				{:else}
					No result yet.
				{/if}
			</CardFooter>
		</Card>
	</Container>
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
