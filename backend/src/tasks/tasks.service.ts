import { Injectable } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Task } from './task.entity';
import { Execution } from './execution.entity';

@Injectable()
export class TasksService {
  constructor(
    @InjectRepository(Task) private tasks: Repository<Task>,
    @InjectRepository(Execution) private executions: Repository<Execution>,
  ) {}

  findAll() { return this.tasks.find({ order: { createdAt: 'DESC' } }); }

  findOne(taskId: number) { return this.tasks.findOneBy({ taskId }); }

  upsert(data: Partial<Task>) { return this.tasks.save(data); }

  findExecutions(taskId: number) {
    return this.executions.findBy({ taskId });
  }

  recordExecution(data: Partial<Execution>) { return this.executions.save(data); }
}
