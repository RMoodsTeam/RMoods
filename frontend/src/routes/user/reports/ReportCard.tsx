import React from 'react';
import { Card, Title, Text, Checkbox, Grid } from '@mantine/core';

interface Report {
    id: string;
    user_id: string;
    title: string;
    description: string;
    is_public: boolean;
    status: {
        Error?: string;
    };
    analyses: Record<string, unknown>;
    created_at: string;
    updated_at: string;
}

interface ReportCardProps {
    report: Report;
    onCheck: (id: string, checked: boolean) => void;
    checked: boolean;
}

const ReportCard: React.FC<ReportCardProps> = ({ report, onCheck, checked }) => {
    const handleCheck = (event: React.ChangeEvent<HTMLInputElement>) => {
        onCheck(report.id, event.target.checked);
    };

    return (
        <Card shadow="sm" padding="lg" radius='md' style={{ marginBottom: '10px', display: 'flex' }}>
            <Grid align="center">
                <Grid.Col span={1}>
                    <Checkbox onChange={handleCheck} checked={checked} />
                </Grid.Col>
                <Grid.Col span={3}>
                    <Title order={3}>{report.title}</Title>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Text>{report.description}</Text>
                </Grid.Col>
                <Grid.Col span={4}>
                    <Text>{new Date(report.created_at).toLocaleString()}</Text>
                </Grid.Col>
            </Grid>
        </Card>
    );
};

export default ReportCard;