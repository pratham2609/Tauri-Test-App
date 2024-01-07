'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'

// To call our newly created command we will use the @tauri-apps/api JavaScript library. 
// It provides access to core functionality such as window manipulation, the filesystem, and more through convenient JavaScript abstractions.
export default function Greet() {
    const [greeting, setGreeting] = useState('');

    useEffect(() => {
        invoke<string>('greet', { name: 'Pratham' })
            .then(result => setGreeting(result))
            .catch(console.error)
    }, [])

    // Necessary because we will have to use Greet as a component later.
    return <div>{greeting}</div>;
}