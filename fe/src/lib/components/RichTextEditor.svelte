<script lang="ts">
    import { onMount, onDestroy, createEventDispatcher } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Link from '@tiptap/extension-link';
    import Placeholder from '@tiptap/extension-placeholder';

    export let content = '';
    export let placeholder = 'Write something...';
    export let editable = true;

    const dispatch = createEventDispatcher<{ change: string }>();

    let element: HTMLDivElement;
    let editor: Editor | null = null;

    // Toolbar state
    let isBold = false;
    let isItalic = false;
    let isStrike = false;
    let isCode = false;
    let isBulletList = false;
    let isOrderedList = false;
    let isBlockquote = false;
    let isCodeBlock = false;
    let isHeading1 = false;
    let isHeading2 = false;
    let isHeading3 = false;

    function updateToolbarState() {
        if (!editor) return;
        isBold = editor.isActive('bold');
        isItalic = editor.isActive('italic');
        isStrike = editor.isActive('strike');
        isCode = editor.isActive('code');
        isBulletList = editor.isActive('bulletList');
        isOrderedList = editor.isActive('orderedList');
        isBlockquote = editor.isActive('blockquote');
        isCodeBlock = editor.isActive('codeBlock');
        isHeading1 = editor.isActive('heading', { level: 1 });
        isHeading2 = editor.isActive('heading', { level: 2 });
        isHeading3 = editor.isActive('heading', { level: 3 });
    }

    onMount(() => {
        editor = new Editor({
            element,
            extensions: [
                StarterKit,
                Link.configure({
                    openOnClick: false,
                    HTMLAttributes: {
                        class: 'link link-primary',
                    },
                }),
                Placeholder.configure({
                    placeholder,
                }),
            ],
            content,
            editable,
            onTransaction: () => {
                updateToolbarState();
            },
            onUpdate: ({ editor: e }) => {
                const html = e.getHTML();
                dispatch('change', html);
            },
            editorProps: {
                attributes: {
                    class: 'prose prose-sm max-w-none focus:outline-none min-h-[120px] p-3',
                },
            },
        });
    });

    onDestroy(() => {
        editor?.destroy();
    });

    function toggleBold() {
        editor?.chain().focus().toggleBold().run();
    }
    function toggleItalic() {
        editor?.chain().focus().toggleItalic().run();
    }
    function toggleStrike() {
        editor?.chain().focus().toggleStrike().run();
    }
    function toggleCode() {
        editor?.chain().focus().toggleCode().run();
    }
    function toggleBulletList() {
        editor?.chain().focus().toggleBulletList().run();
    }
    function toggleOrderedList() {
        editor?.chain().focus().toggleOrderedList().run();
    }
    function toggleBlockquote() {
        editor?.chain().focus().toggleBlockquote().run();
    }
    function toggleCodeBlock() {
        editor?.chain().focus().toggleCodeBlock().run();
    }
    function setHeading(level: 1 | 2 | 3) {
        editor?.chain().focus().toggleHeading({ level }).run();
    }
    function setParagraph() {
        editor?.chain().focus().setParagraph().run();
    }
    function addLink() {
        const url = prompt('Enter URL:');
        if (url) {
            editor?.chain().focus().setLink({ href: url }).run();
        }
    }
    function removeLink() {
        editor?.chain().focus().unsetLink().run();
    }
</script>

<div class="rte-wrapper border border-base-300 rounded-lg overflow-hidden bg-base-100">
    {#if editable}
        <div class="rte-toolbar flex flex-wrap gap-0.5 p-1.5 border-b border-base-300 bg-base-200">
            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBold}
                    on:click={toggleBold}
                    title="Bold">
                    <strong>B</strong>
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isItalic}
                    on:click={toggleItalic}
                    title="Italic">
                    <em>I</em>
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isStrike}
                    on:click={toggleStrike}
                    title="Strikethrough">
                    <s>S</s>
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isCode}
                    on:click={toggleCode}
                    title="Inline Code">
                    <span class="font-mono text-xs">&lt;/&gt;</span>
                </button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isHeading1}
                    on:click={() => setHeading(1)}
                    title="Heading 1">
                    H1
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isHeading2}
                    on:click={() => setHeading(2)}
                    title="Heading 2">
                    H2
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isHeading3}
                    on:click={() => setHeading(3)}
                    title="Heading 3">
                    H3
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={setParagraph}
                    title="Paragraph">
                    ¶
                </button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBulletList}
                    on:click={toggleBulletList}
                    title="Bullet List">
                    •≡
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isOrderedList}
                    on:click={toggleOrderedList}
                    title="Ordered List">
                    1.
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBlockquote}
                    on:click={toggleBlockquote}
                    title="Blockquote">
                    ❝
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isCodeBlock}
                    on:click={toggleCodeBlock}
                    title="Code Block">
                    { '{ }' }
                </button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={addLink}
                    title="Add Link">
                    🔗
                </button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={removeLink}
                    title="Remove Link">
                    🔗̸
                </button>
            </div>
        </div>
    {/if}

    <div bind:this={element}></div>
</div>

<style>
    .rte-wrapper :global(.tiptap p.is-editor-empty:first-child::before) {
        content: attr(data-placeholder);
        float: left;
        color: oklch(var(--bc) / 0.4);
        pointer-events: none;
        height: 0;
    }

    .rte-wrapper :global(.tiptap) {
        outline: none;
    }

    .rte-wrapper :global(.tiptap:focus) {
        outline: none;
    }
</style>
