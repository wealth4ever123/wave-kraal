import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { Keeper } from './keeper.entity';
import { KeepersController } from './keepers.controller';
import { KeepersService } from './keepers.service';

@Module({
  imports: [TypeOrmModule.forFeature([Keeper])],
  controllers: [KeepersController],
  providers: [KeepersService],
  exports: [KeepersService],
})
export class KeepersModule {}
