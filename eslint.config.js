/** @type { import("eslint").Linter.Config } */
import { FlatCompat } from '@eslint/eslintrc'
import path from 'path'
import { fileURLToPath } from 'url'
import js from '@eslint/js'

// mimic CommonJS variables -- not needed if using CommonJS
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

const compat = new FlatCompat({
    baseDirectory: __dirname, // optional; default: process.cwd()
    resolvePluginsRelativeTo: __dirname, // optional
    recommendedConfig: js
})

export default [
    ...compat.config({
        root: true,
        extends: ['plugin:@typescript-eslint/recommended', 'plugin:svelte/recommended', 'prettier'],
        parser: '@typescript-eslint/parser',
        plugins: ['@typescript-eslint'],
        parserOptions: {
            sourceType: 'module',
            ecmaVersion: 2020,
            extraFileExtensions: ['.svelte']
        },
        env: {
            browser: true,
            es2017: true,
            node: true
        },
        overrides: [
            {
                files: ['*.svelte'],
                parser: 'svelte-eslint-parser',
                parserOptions: {
                    parser: '@typescript-eslint/parser'
                },
                rules: {
                    'svelte/no-at-html-tags': 'off'
                }
            }
        ],
        ignorePatterns: ['src-tauri/*', '*.lock*', 'node_modules/*', 'build/*', '.eslintcache']
    })
]
