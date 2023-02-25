// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    modules: [
        '@huntersofbook/naive-ui-nuxt',
        '@nuxtjs/tailwindcss',
        'nuxt-icon',
        [
            '@pinia/nuxt',
            {
                autoImports: ['defineStore', 'acceptHMRUpdate'],
            }
        ]
    ],
    imports: {
        // Auto-import pinia stores defined in `~/stores`
        dirs: ['stores']
    }
})
