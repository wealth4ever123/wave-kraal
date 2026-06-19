import { Controller, Get, Param, ParseIntPipe } from '@nestjs/common';
import { TasksService } from './tasks.service';

@Controller('tasks')
export class TasksController {
  constructor(private readonly tasks: TasksService) {}

  @Get()
  findAll() { return this.tasks.findAll(); }

  @Get(':id')
  findOne(@Param('id', ParseIntPipe) id: number) { return this.tasks.findOne(id); }

  @Get(':id/executions')
  executions(@Param('id', ParseIntPipe) id: number) { return this.tasks.findExecutions(id); }
}
