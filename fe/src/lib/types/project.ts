export interface Project {
    id: string;
    project_key: string;
    name: string;
    description: string | null;
    status: string;
    created_by: string | null;
    created_at: string;
    updated_at: string;
    member_role?: string | null;
}

export interface CreateProjectDto {
    project_key: string;
    name: string;
    description?: string;
}

export interface UpdateProjectDto {
    name?: string;
    description?: string;
    status?: string;
}

export interface ProjectMember {
    id: string;
    project_id: string;
    user_id: string;
    user_name: string;
    email: string;
    role: string;
    created_at: string;
}

export interface BoardColumn {
    id: string;
    board_id: string;
    name: string;
    display_order: number;
    created_at: string;
    updated_at: string;
}

export interface Board {
    id: string;
    project_id: string;
    name: string;
    created_at: string;
    updated_at: string;
    columns: BoardColumn[];
}

export interface Card {
    id: string;
    project_id: string;
    column_id: string | null;
    title: string;
    description: string | null;
    priority: string;
    assignee_id: string | null;
    assignee_name?: string | null;
    due_date: string | null;
    display_order: number;
    created_at: string;
    updated_at: string;
}

export interface CreateCardDto {
    column_id?: string;
    title: string;
    description?: string;
    priority?: string;
    assignee_id?: string;
    due_date?: string;
    display_order?: number;
}

export interface UpdateCardDto {
    column_id?: string;
    title?: string;
    description?: string;
    priority?: string;
    assignee_id?: string;
    due_date?: string;
    display_order?: number;
}
