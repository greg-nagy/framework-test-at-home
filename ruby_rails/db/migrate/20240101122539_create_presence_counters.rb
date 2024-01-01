class CreatePresenceCounters < ActiveRecord::Migration[7.1]
  def change
    create_table :presence_counters do |t|
      t.string :name
      t.integer :count
      t.datetime :updated_at

      t.timestamps
    end
  end
end
