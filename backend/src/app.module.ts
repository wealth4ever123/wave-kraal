import { Module } from '@nestjs/common';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { TypeOrmModule } from '@nestjs/typeorm';
import { TasksModule } from './tasks/tasks.module';
import { KeepersModule } from './keepers/keepers.module';
import { Task } from './tasks/task.entity';
import { Keeper } from './keepers/keeper.entity';
import { Execution } from './tasks/execution.entity';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    TypeOrmModule.forRootAsync({
      inject: [ConfigService],
      useFactory: (cfg: ConfigService) => ({
        type: 'postgres',
        url: cfg.get('DATABASE_URL'),
        entities: [Task, Keeper, Execution],
        synchronize: true,
      }),
    }),
    TasksModule,
    KeepersModule,
  ],
})
export class AppModule {}
