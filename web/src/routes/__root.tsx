import {createRootRoute, HeadContent, Scripts} from '@tanstack/react-router'
import {TanStackRouterDevtoolsPanel} from '@tanstack/react-router-devtools'
import {TanStackDevtools} from '@tanstack/react-devtools'

import Header from '../components/Header'

import appCss from '../styles.css?url'

export const Route = createRootRoute({
    head: () => ({
        meta: [
            {
                charSet: 'utf-8',
            },
            {
                name: 'viewport',
                content: 'width=device-width, initial-scale=1',
            },
            {
                title: 'AXIOM | RPG Social e Engajamento para Discord',
            },
            {
                name: 'description',
                content: 'Transforme conversas em batalhas épicas. O AXIOM é um bot de Discord feito em Rust que cria Raids baseadas na interação social do seu servidor. Sem spam de comandos.',
            },
            {
                name: 'keywords',
                content: 'Discord Bot, RPG, Rust, Social Gaming, Engajamento, Raid',
            },
            {
                name: 'theme-color',
                content: '#000000',
            },
            {
                property: 'og:type',
                content: 'website',
            },
            {
                property: 'og:url',
                content: 'https://axiombot.com.br',
            },
            {
                property: 'og:title',
                content: 'AXIOM: O Sistema está Online.',
            },
            {
                property: 'og:description',
                content: 'Não apenas converse, jogue. Adicione a camada de RPG Social definitiva ao seu servidor.',
            },
            {
                property: 'og:image',
                content: 'https://axiombot.com.br/og-banner.png',
            },
            {
                property: 'twitter:card',
                content: 'summary_large_image',
            },
        ],
        links: [
            {
                rel: 'stylesheet',
                href: appCss,
            },
        ],
    }),

    shellComponent: RootDocument,
})

function RootDocument({children}: { children: React.ReactNode }) {
    return (
        <html lang="en">
        <head>
            <HeadContent/>
        </head>
        <body>
        <Header/>
        {children}
        <TanStackDevtools
            config={{
                position: 'bottom-right',
            }}
            plugins={[
                {
                    name: 'Tanstack Router',
                    render: <TanStackRouterDevtoolsPanel/>,
                },
            ]}
        />
        <Scripts/>
        </body>
        </html>
    )
}
