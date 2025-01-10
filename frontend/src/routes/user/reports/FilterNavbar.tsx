import React from 'react';
import {
  ActionIcon,
  Box,
  Card,
  Center,
  Grid,
  Group,
  MultiSelect,
  Select,
  TextInput,
} from '@mantine/core';
import { DatePickerInput } from '@mantine/dates';
import { NlpAnalysisKind, NlpAnalysisKindUtil } from '../../../rmoods/types.ts';
import '@mantine/dates/styles.css';
import { IconFilterCancel } from '@tabler/icons-react';
import { MyReportsPageReportQuery } from './page.tsx';

interface FilterNavbarProps {
  filters: MyReportsPageReportQuery;
  onFilterChange: (field: keyof MyReportsPageReportQuery, value: any) => void;
  onClearFilters: () => void;
}

const FilterNavbar: React.FC<FilterNavbarProps> = ({
  filters,
  onFilterChange,
  onClearFilters,
}) => {
  const handleNlpKindChange = (kind: NlpAnalysisKind, checked: boolean) => {
    onFilterChange(
      'containedAnalysisKinds',
      checked
        ? [...filters.containedAnalysisKinds, kind]
        : filters.containedAnalysisKinds.filter((k) => k !== kind)
    );
  };

  return (
    <Box>
      <Card>
        <Center>
          <Box style={{ marginBottom: '15 px', width: '50em' }}>
            <Grid justify="center" grow>
              <Grid.Col span={4}>
                <TextInput
                  placeholder="eg. My report"
                  label="Title pattern"
                  value={filters.titlePattern}
                  onChange={(event) =>
                    onFilterChange('titlePattern', event.currentTarget.value)
                  }
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <DatePickerInput
                  label="From date"
                  placeholder="eg. 2024-12-01"
                  value={filters.startDate}
                  clearable
                  onChange={(date) => {
                    onFilterChange('startDate', date);
                  }}
                  minDate={new Date(2024, 11, 1)}
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <DatePickerInput
                  label="To date"
                  placeholder="eg. 2024-12-31"
                  value={filters.endDate}
                  clearable
                  onChange={(date) => {
                    onFilterChange('endDate', date);
                  }}
                />
              </Grid.Col>
              <Grid.Col span={4}>
                <MultiSelect
                  wrapperProps={{ style: { width: '100%' } }}
                  data={Object.values(NlpAnalysisKind).map((kind) => ({
                    value: kind,
                    label: NlpAnalysisKindUtil.getDisplayLabel(kind),
                  }))}
                  label="NLP Analyses"
                  value={filters.containedAnalysisKinds}
                  onChange={(value) =>
                    onFilterChange(
                      'containedAnalysisKinds',
                      value as NlpAnalysisKind[]
                    )
                  }
                />
              </Grid.Col>
            </Grid>
          </Box>
        </Center>
      </Card>
      <Group justify="right" style={{ marginTop: '20px' }} align={'end'}>
        <ActionIcon onClick={onClearFilters} size={'lg'}>
          <IconFilterCancel />
        </ActionIcon>
        <Select
          label="Per page"
          onChange={(value) =>
            onFilterChange('perPage', Number.parseInt(value!, 10))
          }
          w={'5em'}
          defaultValue={filters.perPage.toString()}
          allowDeselect={false}
          data={[20, 30, 50, 100].map((value) => value.toString())}
        />
      </Group>
    </Box>
  );
};

export default FilterNavbar;
