<script lang="ts">
    import { onMount, onDestroy, createEventDispatcher } from 'svelte';
    import { Editor } from '@tiptap/core';
    import StarterKit from '@tiptap/starter-kit';
    import Underline from '@tiptap/extension-underline';
    import Link from '@tiptap/extension-link';
    import Placeholder from '@tiptap/extension-placeholder';
    import { TextStyle } from '@tiptap/extension-text-style';
    import Color from '@tiptap/extension-color';
    import FontFamily from '@tiptap/extension-font-family';
    import Highlight from '@tiptap/extension-highlight';
    import TextAlign from '@tiptap/extension-text-align';
    import Subscript from '@tiptap/extension-subscript';
    import Superscript from '@tiptap/extension-superscript';
    import ImageExt from '@tiptap/extension-image';
    import { Table } from '@tiptap/extension-table';
    import TableRow from '@tiptap/extension-table-row';
    import TableHeader from '@tiptap/extension-table-header';
    import TableCell from '@tiptap/extension-table-cell';
    import TaskList from '@tiptap/extension-task-list';
    import TaskItem from '@tiptap/extension-task-item';

    export let content = '';
    export let placeholder = 'Write something...';
    export let editable = true;

    const dispatch = createEventDispatcher<{ change: string }>();

    let element: HTMLDivElement;
    let editor: Editor | null = null;

    // Toolbar state
    let isBold = false;
    let isItalic = false;
    let isUnderline = false;
    let isStrike = false;
    let isCode = false;
    let isSubscript = false;
    let isSuperscript = false;
    let isBulletList = false;
    let isOrderedList = false;
    let isTaskList = false;
    let isBlockquote = false;
    let isCodeBlock = false;
    let isHeading1 = false;
    let isHeading2 = false;
    let isHeading3 = false;
    let isAlignLeft = false;
    let isAlignCenter = false;
    let isAlignRight = false;
    let isAlignJustify = false;
    let isLink = false;
    let inTable = false;
    let currentColor = '#000000';
    let currentHighlight = '#fef08a';

    const FONT_FAMILIES = [
        { label: 'Default', value: '' },
        { label: 'Sans Serif', value: 'ui-sans-serif, system-ui, sans-serif' },
        { label: 'Serif', value: 'ui-serif, Georgia, serif' },
        { label: 'Monospace', value: 'ui-monospace, monospace' },
        { label: 'Arial', value: 'Arial, sans-serif' },
        { label: 'Georgia', value: 'Georgia, serif' },
        { label: 'Times New Roman', value: '"Times New Roman", serif' },
        { label: 'Courier New', value: '"Courier New", monospace' },
    ];

    const TEXT_COLORS = [
        '#000000', '#374151', '#dc2626', '#ea580c', '#ca8a04',
        '#16a34a', '#0891b2', '#2563eb', '#7c3aed', '#db2777',
    ];

    const HIGHLIGHT_COLORS = [
        '#fef08a', '#bbf7d0', '#bfdbfe', '#fbcfe8', '#fed7aa', '#e9d5ff',
    ];

    function updateToolbarState() {
        if (!editor) return;
        isBold = editor.isActive('bold');
        isItalic = editor.isActive('italic');
        isUnderline = editor.isActive('underline');
        isStrike = editor.isActive('strike');
        isCode = editor.isActive('code');
        isSubscript = editor.isActive('subscript');
        isSuperscript = editor.isActive('superscript');
        isBulletList = editor.isActive('bulletList');
        isOrderedList = editor.isActive('orderedList');
        isTaskList = editor.isActive('taskList');
        isBlockquote = editor.isActive('blockquote');
        isCodeBlock = editor.isActive('codeBlock');
        isHeading1 = editor.isActive('heading', { level: 1 });
        isHeading2 = editor.isActive('heading', { level: 2 });
        isHeading3 = editor.isActive('heading', { level: 3 });
        isAlignLeft = editor.isActive({ textAlign: 'left' });
        isAlignCenter = editor.isActive({ textAlign: 'center' });
        isAlignRight = editor.isActive({ textAlign: 'right' });
        isAlignJustify = editor.isActive({ textAlign: 'justify' });
        isLink = editor.isActive('link');
        inTable = editor.isActive('table');
        currentColor = editor.getAttributes('textStyle').color || '#000000';
        currentHighlight = editor.getAttributes('highlight').color || '#fef08a';
    }

    onMount(() => {
        editor = new Editor({
            element,
            extensions: [
                StarterKit,
                Underline,
                TextStyle,
                Color,
                FontFamily,
                Highlight.configure({ multicolor: true }),
                TextAlign.configure({ types: ['heading', 'paragraph'] }),
                Subscript,
                Superscript,
                ImageExt.configure({ HTMLAttributes: { class: 'rounded-lg max-w-full' } }),
                Table.configure({ resizable: true }),
                TableRow,
                TableHeader,
                TableCell,
                TaskList,
                TaskItem.configure({ nested: true }),
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
                    class: 'prose prose-sm max-w-none focus:outline-none min-h-[280px] p-4',
                },
            },
        });
        updateToolbarState();
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
    function toggleUnderline() {
        editor?.chain().focus().toggleUnderline().run();
    }
    function toggleStrike() {
        editor?.chain().focus().toggleStrike().run();
    }
    function toggleCode() {
        editor?.chain().focus().toggleCode().run();
    }
    function toggleSubscript() {
        editor?.chain().focus().unsetSuperscript().toggleSubscript().run();
    }
    function toggleSuperscript() {
        editor?.chain().focus().unsetSubscript().toggleSuperscript().run();
    }
    function toggleBulletList() {
        editor?.chain().focus().toggleBulletList().run();
    }
    function toggleOrderedList() {
        editor?.chain().focus().toggleOrderedList().run();
    }
    function toggleTaskList() {
        editor?.chain().focus().toggleTaskList().run();
    }
    function toggleBlockquote() {
        editor?.chain().focus().toggleBlockquote().run();
    }
    function toggleCodeBlock() {
        editor?.chain().focus().toggleCodeBlock().run();
    }
    function setHeading(level: 1 | 2 | 3 | 4 | 5 | 6) {
        editor?.chain().focus().toggleHeading({ level }).run();
    }
    function setParagraph() {
        editor?.chain().focus().setParagraph().run();
    }
    function onBlockTypeChange(e: Event) {
        const value = (e.target as HTMLSelectElement).value;
        if (value === 'p') setParagraph();
        else setHeading(Number(value.slice(1)) as 1 | 2 | 3 | 4 | 5 | 6);
    }
    function setAlign(align: 'left' | 'center' | 'right' | 'justify') {
        editor?.chain().focus().setTextAlign(align).run();
    }
    function setFontFamily(e: Event) {
        const value = (e.target as HTMLSelectElement).value;
        if (value) {
            editor?.chain().focus().setFontFamily(value).run();
        } else {
            editor?.chain().focus().unsetFontFamily().run();
        }
    }
    function setColor(color: string) {
        editor?.chain().focus().setColor(color).run();
    }
    function setHighlight(color: string) {
        editor?.chain().focus().toggleHighlight({ color }).run();
    }
    function unsetHighlight() {
        editor?.chain().focus().unsetHighlight().run();
    }
    function addLink() {
        const previousUrl = editor?.getAttributes('link').href;
        const url = prompt('Enter URL:', previousUrl || 'https://');
        if (url === null) return;
        if (url === '') {
            editor?.chain().focus().unsetLink().run();
            return;
        }
        editor?.chain().focus().setLink({ href: url }).run();
    }
    function removeLink() {
        editor?.chain().focus().unsetLink().run();
    }
    function addImage() {
        const url = prompt('Image URL:');
        if (url) {
            editor?.chain().focus().setImage({ src: url }).run();
        }
    }
    function setHorizontalRule() {
        editor?.chain().focus().setHorizontalRule().run();
    }
    function insertTable() {
        editor?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run();
    }
    function addColumnAfter() {
        editor?.chain().focus().addColumnAfter().run();
    }
    function addRowAfter() {
        editor?.chain().focus().addRowAfter().run();
    }
    function deleteColumn() {
        editor?.chain().focus().deleteColumn().run();
    }
    function deleteRow() {
        editor?.chain().focus().deleteRow().run();
    }
    function deleteTable() {
        editor?.chain().focus().deleteTable().run();
    }
    function undo() {
        editor?.chain().focus().undo().run();
    }
    function redo() {
        editor?.chain().focus().redo().run();
    }
    function clearFormatting() {
        editor?.chain().focus().unsetAllMarks().clearNodes().run();
    }

    $: currentBlockType = isHeading1
        ? 'h1'
        : isHeading2
          ? 'h2'
          : isHeading3
            ? 'h3'
            : 'p';
</script>

<div class="rte-wrapper border border-base-300 rounded-lg overflow-hidden bg-base-100">
    {#if editable}
        <div class="rte-toolbar flex flex-wrap items-center gap-1 p-2 border-b border-base-300 bg-base-200">
            <!-- History -->
            <div class="join">
                <button type="button" class="btn btn-xs join-item" on:click={undo} title="Undo">↶</button>
                <button type="button" class="btn btn-xs join-item" on:click={redo} title="Redo">↷</button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Block type -->
            <select
                class="select select-bordered select-xs w-28"
                value={currentBlockType}
                on:change={onBlockTypeChange}
                title="Paragraph style">
                <option value="p">Paragraph</option>
                <option value="h1">Heading 1</option>
                <option value="h2">Heading 2</option>
                <option value="h3">Heading 3</option>
            </select>

            <!-- Font family -->
            <select
                class="select select-bordered select-xs w-28"
                on:change={setFontFamily}
                title="Font family">
                {#each FONT_FAMILIES as f}
                    <option value={f.value}>{f.label}</option>
                {/each}
            </select>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Inline marks -->
            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBold}
                    on:click={toggleBold}
                    title="Bold"><strong>B</strong></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isItalic}
                    on:click={toggleItalic}
                    title="Italic"><em>I</em></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isUnderline}
                    on:click={toggleUnderline}
                    title="Underline"><span class="underline">U</span></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isStrike}
                    on:click={toggleStrike}
                    title="Strikethrough"><s>S</s></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isCode}
                    on:click={toggleCode}
                    title="Inline Code"><span class="font-mono text-xs">&lt;/&gt;</span></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isSubscript}
                    on:click={toggleSubscript}
                    title="Subscript">X<sub>2</sub></button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isSuperscript}
                    on:click={toggleSuperscript}
                    title="Superscript">X<sup>2</sup></button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Text color -->
            <div class="dropdown">
                <button
                    type="button"
                    tabindex="0"
                    class="btn btn-xs"
                    title="Text color">
                    A<span class="w-3 h-1 block" style="background:{currentColor}"></span>
                </button>
                <div class="dropdown-content z-20 bg-base-100 border border-base-300 rounded-lg p-2 shadow-xl flex flex-wrap gap-1 w-32">
                    {#each TEXT_COLORS as c}
                        <button
                            type="button"
                            class="w-5 h-5 rounded border border-base-300"
                            style="background:{c}"
                            title={c}
                            aria-label="Text color {c}"
                            on:click={() => setColor(c)}></button>
                    {/each}
                </div>
            </div>

            <!-- Highlight color -->
            <div class="dropdown">
                <button
                    type="button"
                    tabindex="0"
                    class="btn btn-xs"
                    title="Highlight color">
                    <span class="px-0.5" style="background:{currentHighlight}">H</span>
                </button>
                <div class="dropdown-content z-20 bg-base-100 border border-base-300 rounded-lg p-2 shadow-xl flex flex-wrap gap-1 w-32">
                    {#each HIGHLIGHT_COLORS as c}
                        <button
                            type="button"
                            class="w-5 h-5 rounded border border-base-300"
                            style="background:{c}"
                            title={c}
                            aria-label="Highlight color {c}"
                            on:click={() => setHighlight(c)}></button>
                    {/each}
                    <button
                        type="button"
                        class="btn btn-3xs w-full mt-1"
                        on:click={unsetHighlight}>None</button>
                </div>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Alignment -->
            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isAlignLeft}
                    on:click={() => setAlign('left')}
                    title="Align left">≡</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isAlignCenter}
                    on:click={() => setAlign('center')}
                    title="Align center">≣</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isAlignRight}
                    on:click={() => setAlign('right')}
                    title="Align right">≢</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isAlignJustify}
                    on:click={() => setAlign('justify')}
                    title="Justify">☰</button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Lists -->
            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBulletList}
                    on:click={toggleBulletList}
                    title="Bullet List">•≡</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isOrderedList}
                    on:click={toggleOrderedList}
                    title="Ordered List">1.</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isTaskList}
                    on:click={toggleTaskList}
                    title="Checklist">☑</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isBlockquote}
                    on:click={toggleBlockquote}
                    title="Blockquote">❝</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isCodeBlock}
                    on:click={toggleCodeBlock}
                    title="Code Block">{'{ }'}</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={setHorizontalRule}
                    title="Horizontal rule">―</button>
            </div>

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <!-- Link / image / table -->
            <div class="join">
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    class:btn-active={isLink}
                    on:click={addLink}
                    title="Add Link">🔗</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={removeLink}
                    title="Remove Link">🔗̸</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={addImage}
                    title="Insert image">🖼</button>
                <button
                    type="button"
                    class="btn btn-xs join-item"
                    on:click={insertTable}
                    title="Insert table">⊞</button>
            </div>

            {#if inTable}
                <div class="join">
                    <button type="button" class="btn btn-xs join-item" on:click={addRowAfter} title="Add row">+row</button>
                    <button type="button" class="btn btn-xs join-item" on:click={addColumnAfter} title="Add column">+col</button>
                    <button type="button" class="btn btn-xs join-item" on:click={deleteRow} title="Delete row">-row</button>
                    <button type="button" class="btn btn-xs join-item" on:click={deleteColumn} title="Delete column">-col</button>
                    <button type="button" class="btn btn-xs join-item text-error" on:click={deleteTable} title="Delete table">✕table</button>
                </div>
            {/if}

            <div class="divider divider-horizontal mx-0.5 w-0"></div>

            <button
                type="button"
                class="btn btn-xs"
                on:click={clearFormatting}
                title="Clear formatting">Clear</button>
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

    .rte-wrapper :global(.tiptap table) {
        border-collapse: collapse;
        table-layout: fixed;
        width: 100%;
        margin: 0.5rem 0;
        overflow: hidden;
    }

    .rte-wrapper :global(.tiptap table td),
    .rte-wrapper :global(.tiptap table th) {
        border: 1px solid oklch(var(--bc) / 0.2);
        padding: 0.35rem 0.5rem;
        vertical-align: top;
        position: relative;
    }

    .rte-wrapper :global(.tiptap table th) {
        background: oklch(var(--bc) / 0.06);
        font-weight: 600;
        text-align: left;
    }

    .rte-wrapper :global(.tiptap ul[data-type='taskList']) {
        list-style: none;
        padding-left: 0;
    }

    .rte-wrapper :global(.tiptap ul[data-type='taskList'] li) {
        display: flex;
        align-items: flex-start;
        gap: 0.4rem;
    }

    .rte-wrapper :global(.tiptap ul[data-type='taskList'] li > label) {
        margin-top: 0.2rem;
    }
</style>
