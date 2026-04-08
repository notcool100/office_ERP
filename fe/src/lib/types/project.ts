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
    sequence_no: number;
    card_key: string;
    title: string;
    description: string | null;
    sprint_name: string | null;
    priority: string;
    assignee_id: string | null;
    assignee_name?: string | null;
    due_date: string | null;
    display_order: number;
    created_at: string;
    updated_at: string;
    sprint_id: string | null;
}

export interface Sprint {
    id: string;
    project_id: string;
    name: string;
    goal: string | null;
    start_date: string | null;
    end_date: string | null;
    status: string;
    created_at: string;
    updated_at: string;
}

export interface CreateSprintDto {
    name: string;
    goal?: string;
    start_date?: string;
    end_date?: string;
}

export interface UpdateSprintDto {
    name?: string;
    goal?: string;
    start_date?: string;
    end_date?: string;
    status?: string;
}

export interface CreateCardDto {
    column_id?: string;
    title: string;
    description?: string;
    sprint_name?: string;
    priority?: string;
    assignee_id?: string;
    due_date?: string;
    display_order?: number;
    sprint_id?: string;
}

export interface UpdateCardDto {
    column_id?: string;
    title?: string;
    description?: string;
    sprint_name?: string;
    priority?: string;
    assignee_id?: string;
    due_date?: string;
    display_order?: number;
    sprint_id?: string;
}

export interface CardComment {
    id: string;
    project_id: string;
    card_id: string;
    user_id: string;
    user_name: string;
    comment: string;
    created_at: string;
    updated_at: string;
}

export interface CreateCardCommentDto {
    comment: string;
}

export interface CardAttachment {
    id: string;
    project_id: string;
    card_id: string;
    uploaded_by: string | null;
    uploader_name: string | null;
    file_name: string;
    content_type: string;
    file_size: number;
    created_at: string;
}

export interface CardActivity {
    id: string;
    project_id: string;
    card_id: string;
    actor_id: string | null;
    actor_name: string | null;
    action_type: string;
    description: string;
    created_at: string;
}
