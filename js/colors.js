'use strict';

const colors = {
    basicColors: {
        '--bg-color': '#fafafa',
        '--text-color': '#202020',

        '--link-color': '#a040e0',
        '--inline-code-color': '#EC5E0C',
        '--accent-color': '#a0a0a0',

        '--code-bg-color': 'white',
        '--code-default-color': '#666',
        '--code-name-color': '#EC5E0C',
        '--code-kw-color': '#BA2640'
    },
    warmColors: {
        '--bg-color': '#fafaf8',
        '--text-color': '#302010',

        '--link-color': '#a040e0',
        '--inline-code-color': '#EC5E0C',
        '--accent-color': '#a09090',

        '--code-bg-color': '#fbfbfb',
        '--code-default-color': '#402010',
        '--code-name-color': '#EC5E0C',
        '--code-kw-color': '#BA2640',
    }
};

console.log("colorsets available");

for (const key of Object.keys(colors)) {
    console.log("  ", key);
}

console.log("call changeColors(colors.[colorset-name]) to change");

function changeColors(colors) {
    console.log("colors in this set");
    for (const [key, value] of Object.entries(colors)) {
        console.log("  ", key, value);
        document.documentElement.style.setProperty(key, value);
    }
}
